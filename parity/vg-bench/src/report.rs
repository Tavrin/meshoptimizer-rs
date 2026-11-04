//! Minimal JSON-lines writer and process measurements.
use std::fmt::Display;

/// An ordered JSON object built from displayable values.
#[derive(Clone, Default)]
pub struct Json(Vec<(String, String)>);

impl Json {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a numeric or boolean field (non-finite numbers become null).
    pub fn with<V: Display>(mut self, key: &str, value: V) -> Self {
        let text = value.to_string();
        let ok = text.parse::<f64>().map(|n| n.is_finite()).unwrap_or(false) || text == "true" || text == "false";
        self.0.push((key.into(), if ok { text } else { "null".into() }));
        self
    }

    /// Add a string field.
    pub fn text(mut self, key: &str, value: &str) -> Self {
        let esc: String = value
            .chars()
            .flat_map(|c| match c {
                '"' => vec!['\\', '"'],
                '\\' => vec!['\\', '\\'],
                c if (c as u32) < 0x20 => vec![' '],
                c => vec![c],
            })
            .collect();
        self.0.push((key.into(), format!("\"{esc}\"")));
        self
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, (k, v)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "\"{k}\":{v}")?;
        }
        write!(f, "}}")
    }
}

/// Peak resident set size of this process in KiB, from the kernel high-water mark.
pub fn peak_rss_kib() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|n| n.parse().ok()))
        })
        .unwrap_or(0)
}

/// Size of `bytes` after zstd level 19.
pub fn zstd19(bytes: &[u8]) -> usize {
    zstd::bulk::compress(bytes, 19).map(|v| v.len()).unwrap_or(0)
}
