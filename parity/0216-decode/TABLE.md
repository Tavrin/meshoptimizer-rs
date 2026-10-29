# Decoder default-build measurements

instructions:u; three interleaved repetitions per arm; median; setup, encoding, serialization and warmup excluded. Wall times are disclosure only.

| Case | API | Rust base / C++ default instructions | Final / base instructions | Control / base | Final / C++ scalar | Final / C++ default | Rust base / C++ default wall |
|---|---|---:|---:|---:|---:|---:|---:|
| medium-seams | index-decode | 1.0834 | 1.0000 | 1.0000 | 1.0834 | 1.0834 | 1.1695 |
| medium-seams | vertex-decode | 1.3464 | 1.0000 | 1.0000 | 0.3302 | 1.3464 | 1.1558 |
| medium-smooth | index-decode | 1.0852 | 1.0000 | 1.0000 | 1.0852 | 1.0852 | 1.0988 |
| medium-smooth | vertex-decode | 1.3331 | 1.0000 | 1.0000 | 0.4081 | 1.3331 | 1.1645 |
| medium-sparse | index-decode | 1.0498 | 1.0000 | 1.0000 | 1.0498 | 1.0498 | 1.0276 |
| medium-sparse | vertex-decode | 1.4682 | 1.0000 | 1.0000 | 0.3059 | 1.4682 | 1.1663 |
| million-smooth | index-decode | 1.0835 | 1.0000 | 1.0000 | 1.0835 | 1.0835 | 1.1854 |
| million-smooth | vertex-decode | 1.3545 | 1.0000 | 1.0000 | 0.2819 | 1.3545 | 0.6755 |
| real-214e | index-decode | 0.9561 | 1.0000 | 1.0000 | 0.9561 | 0.9561 | 1.1167 |
| real-214e | vertex-decode | 1.2630 | 1.0000 | 1.0000 | 0.3764 | 1.2630 | 0.9858 |
| real-a082 | index-decode | 1.1746 | 1.0000 | 1.0000 | 1.1746 | 1.1746 | 1.1133 |
| real-a082 | vertex-decode | 1.2486 | 1.0000 | 1.0000 | 0.3628 | 1.2486 | 1.0115 |
| real-b6fd | index-decode | 0.9567 | 1.0000 | 1.0000 | 0.9567 | 0.9567 | 1.1752 |
| real-b6fd | vertex-decode | 1.2581 | 1.0000 | 1.0000 | 0.3808 | 1.2581 | 0.9214 |
| tiny-smooth | index-decode | 1.1277 | 1.0000 | 1.0000 | 1.1277 | 1.1277 | 1.1301 |
| tiny-smooth | vertex-decode | 1.5572 | 1.0000 | 1.0000 | 0.8013 | 1.5572 | 0.4405 |
