On e-commerce platforms, bot traffic, suspicious transactions, and sudden changes in behavior are typically detected the next day through batch reports, after the damage is done. In this project, the goal is building a system that processes a real e-commerce event stream within seconds. This system consists of two components.

Rust engine: It reads the event stream, filters out and logs erroneous data, detects anomalies using sliding windows, and writes the results.

Python layer: It analyzes and visualizes these results.

