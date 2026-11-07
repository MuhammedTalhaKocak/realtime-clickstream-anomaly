use serde::Deserialize;
use std::time::Instant;
use std::collections::HashMap;

// Aşağıdaki Event Struct'rına derive anahtarı ile debug etme ve deserialize yeteneği kazandıracağız.
// Deserialize internetten, dosyadan gelen ham veriyi Rust Struct'rına çevirme yeteneği. 
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Event {
    event_time: String,
    event_type: String,
    product_id: u64,
    category_id: u64,
    category_code: Option<String>,
    brand: Option<String>,
    price: f64,
    user_id: u64,
    user_session: Option<String>,
}

fn oran(pay:u64, payda:u64) -> Option<f64> {
    if payda == 0 {
        None
    }
    else {
        let yuzde = (pay as f64 / payda as f64) * 100.0;
        Some(yuzde)
    }
}
// Stack (Yığıt) > Boyutu belli, kopyalanması çok hızlı ve kolay olan şeyleri tutar.
// Heap (Yığın) > Boyutu dinamik olan, büyüyüp küçülebilen veya taşınması/kopyalanması maliyetli büyük şeyleri tutar.
// ana fonksiyon hata veya başarılı şeklinde boş unit data type () döndürecek.
// T > Successful
// Box > Rust dilinde RAM'de ne kadar yer kaplayacağı önceden bilinemeyen verileri HEAP'te tutan, elimize de onun adresini (pointer) veren akıllı bir kutudur.
// dyn > Dinamik kelimesinin kısaltmasıdır. Verinin gerçek tipi kod derlenirken değil (compile - time), program çalışırken (run - time) netleşecek.
fn main() ->Result<(), Box<dyn std::error::Error>>{
    let dosya_yolu = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/2019-Nov.csv");
    // concat makrosu > Kendisine verilen birden fazla metni derleme aşamasında uç uca ekleyerek tek bir metin haline getirir.
    // env makrosu >  Env makrosu değeri derleme anında okur. CARGO_MANIFEST_DIR değişkenini de Cargo kendisi ayarlar.
    let mut okuyucu = csv::Reader::from_path(dosya_yolu)?;
    let mut event_type_sayaci: HashMap<String, u64> = HashMap::new();
    let baslangic = Instant::now(); // kodun başladığı süreden itibaren kronometreyi başlatır.
    let mut basarili_islem_sayisi = 0; // başarılı işlem sayısı 0'dan başlıyor.
    let mut hatali_islem_sayisi = 0; // hatalı işlem sayısı 0'dan başlıyor.
    for event in okuyucu.deserialize::<Event>() { // her bir ham satır sırasıyla alınır ve Event struct'runa deserialize edilmeye çalışılır.
        match event { // event tipi ya başarılı dönecek ya da Error olacak. Biz aşağıda match ifadesiyle her iki durumda da RUST'a yapması gerektiğini ileteceğiz.
            Ok(event) => { // eğer event geldiyse, işlem başarılıysa
                if basarili_islem_sayisi < 3 { // başarılı işlem sayısı 3 olana kadar event'i şık bir formatta yazdır ve başarılı işlem sayısını 1 artır.
                    println!("{:#?}", event);
                }
                basarili_islem_sayisi+=1;
                let mevcut_deger = event_type_sayaci.entry(event.event_type).or_insert(0); // Burada ise farklı event type'ları saymak için oluşturduğumuz Hashmap'i kullanıyoruz.
                // burada RUST bize ilgili key'e karşılık gelen value'nın değiştirilebilir referansını verir.
                *mevcut_deger += 1; // Biz de burada dereferencing yaparak value değerini güncelliyoruz.
            }
            Err(hata) => { // eğer herhangi bir durumdan dolayı ham veri Event struct'runa dönüştürülemezse aşağıdaki kod bloğu çalışır.
                if hatali_islem_sayisi < 5 {
                    eprintln!("Hatalı satır atlandı: {}", hata); // eprintln çıktıyı stdout (standart çıktı) ya yazdırmak yerine stderr (standart hata) kısmına yazdırır.
                }
                hatali_islem_sayisi+=1; // hatalı işlem sayısını 1 artırıyoruz.
            }
        }
    } 
    let bitis = baslangic.elapsed(); // Elapsed fonksiyonu kodun bittiği süre ile başladığı süre arasındaki geçen süreyi hesaplar.
    let toplam_islem_sayisi = basarili_islem_sayisi + hatali_islem_sayisi; // hatalı ve başarılı olmak üzere toplam işlem sayısını hesapla.
    println!("---------------------------------");
    println!("Toplam İşlem Sayısı     : {}", toplam_islem_sayisi);
    println!("Başarılı İşlem Sayısı       : {}", basarili_islem_sayisi);
    println!("Hatalı İşlem Sayısı      : {}", hatali_islem_sayisi);
    println!("Geçen süre      : {:?}", bitis);
    println!("Saniyede olay sayısı: {:0}", toplam_islem_sayisi as f64 / bitis.as_secs_f64()); // toplam işlem sayısını f64'e dönüştür.
    // bitis_as_secs_f64 ise Duration nesnesini nanosaniyelerine kadar hesaba katarak toplam saniye cinsinden ondalıklı sayıya dönüştürür.
    // Yukarıdaki formül özünde saniyede toplam kaç işlemin yürütüldüğünü hesaplar.
    let view = event_type_sayaci.get("view").cloned().unwrap_or(0); // view'e karşılık gelen value'nın kendisini alıyoruz, referansla uğraşmamak için. Bunu da cloned ile yapıyoruz, sayılar kopyalanabilir olduğundan belleği yormuyor.
    let purchase = event_type_sayaci.get("purchase").cloned().unwrap_or(0);
    let cart = event_type_sayaci.get("cart").cloned().unwrap_or(0);
    match oran(purchase, view){
        Some(yuzde_degeri) => {
            println!("view → purchase: {:.2}%", yuzde_degeri);
        } // Some bloğunu kapatan parantez ; almaz.
        None => {
            println!("İşlem başarısız: Payda sıfır olduğu için hesaplanamadı.");
        }
    }
    match oran(purchase, cart){
        Some(yuzde_degeri) => {
            println!("cart → purchase: {:.2}%", yuzde_degeri)
        }
        None => {
            println!("İşlem başarısız: Payda sıfır olduğu için hesaplanamadı.");
        }
    }
    match oran(cart, view) {
        Some(yuzde_degeri) => {
            println!("view → cart: {:.2}%", yuzde_degeri);
        }
        None => {
            println!("İşlem başarısız: Payda sıfır olduğu için hesaplanamadı.");
        }
    }
    println!("{:#?}", event_type_sayaci);
    // :? tek satırda yazar. Küçük haritalar için iyidir ancak veri çoksa yan yana uzayıp gider, çıktı okunmaz hale gelebilir.
    // :#? başına koyduğumuz # işareti RUST'a bunu süslü parantezlerle satır satır ve girintili yazdır talimatı verir.
    println!("---------------------------------");
    Ok(())
} 
/*
Benzetme; 5 bardak su içmek için gölün kenarına her seferinde gidip bardağı suyla doldurup içmektense biz
Göle bir defa gittik, kovayı alabildiği kadar su ile doldurduk, evimize geldik. Bu kovadan sadece 5 bardak su içtik işimiz bittiği için de kovada kalan suya dokunmadık.
*/
