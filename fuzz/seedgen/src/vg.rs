//! Reproducible fixed corpus generator. Usage: vg-seedgen OUT_DIR.
#[path = "../../r0220/build_support.rs"]
mod build_support;
#[path = "../../r0220/vg_support.rs"]
mod support;
use std::{fs, path::Path};
fn put(root: &Path, target: &str, name: &str, data: &[u8]) {
    let d = root.join(target);
    fs::create_dir_all(&d).unwrap();
    fs::write(d.join(name), data).unwrap();
}
fn bytes(w: &[u32]) -> Vec<u8> {
    w.iter().flat_map(|w| w.to_le_bytes()).collect()
}
fn checked<T>(root: &Path, target: &str, name: &str, data: &[u8], run: impl FnOnce() -> T) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(run)) {
        Ok(value) => value,
        Err(error) => {
            let artifacts = root.parent().unwrap().join("artifacts").join(target);
            fs::create_dir_all(&artifacts).unwrap();
            let path = artifacts.join(name);
            fs::write(&path, data).unwrap();
            eprintln!(
                "seed assertion failed; input retained at {}",
                path.display()
            );
            std::panic::resume_unwind(error)
        }
    }
}
fn main() {
    let root = std::env::args().nth(1).expect("OUT_DIR");
    let root = Path::new(&root);
    for e in 0..11u8 {
        for variant in 0..3u8 {
            let mut data: Vec<u8> = (0..260)
                .map(|i| ((i * 79 + variant as usize * 17) % 256) as u8)
                .collect();
            data[0] = e;
            data[1] = [0, 13, 253][variant as usize];
            data[2] = variant;
            data[3] = variant;
            let name = format!("encoding-{e}-{variant}");
            let (block, page, file) = checked(root, "attr_page", &name, &data, || {
                support::attribute_fixture(&data)
            });
            put(root, "dag_format_v2", &format!("{name}.mdag"), &file);
            put(root, "attr_page", &format!("{name}.input"), &data);
            put(root, "attr_page", &format!("{name}.block"), &bytes(&block));
            put(root, "attr_page", &format!("{name}.page"), &bytes(&page));
            for codec in [
                meshoptimizer_rs::clusterlod::page::DiskCodec::None,
                meshoptimizer_rs::clusterlod::page::DiskCodec::Compressed,
            ] {
                let disk = meshoptimizer_rs::clusterlod::page::encode_disk_page(
                    &page,
                    meshoptimizer_rs::clusterlod::page::QuantGrid::new(-10).unwrap(),
                    codec,
                    &mut support::ws(),
                )
                .unwrap();
                put(root, "attr_page", &format!("{name}-{codec:?}.disk"), &disk);
            }
        }
    }
    for k in 0..24u8 {
        let mut data: Vec<u8> = (0..180)
            .map(|i| ((i * 13 + k as usize * 17) % 256) as u8)
            .collect();
        data[0] = if k < 16 { 10 } else { k % 11 };
        data[1] = k.wrapping_mul(13);
        put(root, "v2_build", &format!("grid-{k}"), &data);
        checked(root, "v2_build", &format!("grid-{k}"), &data, || {
            build_support::exercise(&data)
        });
    }
}
