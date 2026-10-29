# Decoder default-build measurements

instructions:u; three interleaved repetitions per arm; median; setup, encoding, serialization and warmup excluded. Wall times are disclosure only.

| Case | API | Rust base / C++ default instructions | Final / base instructions | Control / base | Final / C++ scalar | Final / C++ default | Rust base / C++ default wall |
|---|---|---:|---:|---:|---:|---:|---:|
| medium-seams | index-decode | 1.0834 | 1.0132 | 1.0000 | 1.0977 | 1.0977 | 1.1695 |
| medium-seams | vertex-decode | 1.3464 | 0.8795 | 1.0000 | 0.2904 | 1.1841 | 1.1558 |
| medium-smooth | index-decode | 1.0852 | 1.0000 | 1.0000 | 1.0852 | 1.0852 | 1.0988 |
| medium-smooth | vertex-decode | 1.3331 | 0.8851 | 1.0000 | 0.3612 | 1.1800 | 1.1645 |
| medium-sparse | index-decode | 1.0498 | 1.0146 | 1.0000 | 1.0652 | 1.0652 | 1.0276 |
| medium-sparse | vertex-decode | 1.4682 | 0.8361 | 1.0000 | 0.2558 | 1.2276 | 1.1663 |
| million-smooth | index-decode | 1.0835 | 1.0000 | 1.0000 | 1.0835 | 1.0835 | 1.1854 |
| million-smooth | vertex-decode | 1.3545 | 0.8680 | 1.0000 | 0.2447 | 1.1758 | 0.6755 |
| real-214e | index-decode | 0.9561 | 1.0000 | 1.0000 | 0.9561 | 0.9561 | 1.1167 |
| real-214e | vertex-decode | 1.2630 | 0.9090 | 1.0000 | 0.3421 | 1.1481 | 0.9858 |
| real-a082 | index-decode | 1.1746 | 1.0029 | 1.0000 | 1.1781 | 1.1781 | 1.1133 |
| real-a082 | vertex-decode | 1.2486 | 0.9128 | 1.0000 | 0.3311 | 1.1397 | 1.0115 |
| real-b6fd | index-decode | 0.9567 | 1.0000 | 1.0000 | 0.9567 | 0.9567 | 1.1752 |
| real-b6fd | vertex-decode | 1.2581 | 0.9112 | 1.0000 | 0.3470 | 1.1463 | 0.9214 |
| tiny-smooth | index-decode | 1.1277 | 1.0004 | 1.0000 | 1.1281 | 1.1281 | 1.1301 |
| tiny-smooth | vertex-decode | 1.5572 | 0.9296 | 1.0000 | 0.7449 | 1.4476 | 0.4405 |
