// The CLI helpers are private; exercise the actual shared source without
// running its command loop or timing any API.
#![allow(dead_code)]
include!("../src/main.rs");

#[cfg(test)]
mod resident_tests {
    use super::*;
    #[test]
    fn resident_inputs_reuse_storage_and_keep_exact_outputs() {
        let mut resident = None;
        let key = (19, 128, 256, 0);
        let (case, hash) = resident_fixture(&mut resident, key);
        let pointer = case.positions.as_ptr();
        let expected = run("unstripify", key.0, case).unwrap();
        for _ in 0..4 {
            let (case, again) = resident_fixture(&mut resident, key);
            assert_eq!(case.positions.as_ptr(), pointer);
            assert_eq!(again, hash);
            assert_eq!(input_hash(case), hash);
            assert_eq!(run("unstripify", key.0, case).unwrap(), expected);
        }
    }
    #[test]
    fn changed_resident_keys_match_fresh_fixture_bytes() {
        let mut resident = None;
        for key in [
            (19, 128, 256, 0),
            (20, 128, 256, 0),
            (20, 128, 256, 1),
            (20, 128, 256, 2),
            (20, 768, 256, 4),
            (19, 128, 256, 0),
        ] {
            let fresh = case_with(key.0, Some((key.1, key.2, key.3)));
            let (case, hash) = resident_fixture(&mut resident, key);
            assert_eq!(hash, input_hash(&fresh));
            assert_eq!(case.positions, fresh.positions);
            assert_eq!(case.normals, fresh.normals);
            assert_eq!(case.uvs, fresh.uvs);
            assert_eq!(case.indices, fresh.indices);
            assert_eq!(case.texture, fresh.texture);
            assert_eq!(
                run("unstripify", key.0, case).unwrap(),
                run("unstripify", key.0, &fresh).unwrap()
            );
        }
    }
}
