// The CLI helpers are private; exercise the actual shared source without
// running its command loop or timing any API.
#![allow(dead_code)]
include!("../src/main.rs");

#[cfg(test)]
mod resident_tests {
    use super::*;
    #[test]
    fn presized_compact_batch_keeps_legacy_bytes_and_output_tails() {
        // Six frozen seeds plus 32 extra seeds, both state counts. The legacy
        // runner serializer includes speculative data and unused array tails.
        for seed in 0..38 {
            let case = case_with(seed, None);
            let states = if seed & 1 == 0 { 2 } else { 4 };
            let (mut data, mut offsets) = compact_batch_data(&case.texture, states);
            assert_eq!(data.len(), if states == 2 { 12 } else { 22 });
            let mut levels = [0, 1, 2, 3];
            let mut indices = [0, 1, 2, 3, 0, 2];
            let (count, size) = opacity_map_compact(
                &mut data,
                &mut levels,
                &mut offsets,
                &mut indices,
                states,
                &mut Workspace::default(),
            )
            .unwrap();
            let mut out = Vec::new();
            push_u64(&mut out, count);
            push_u64(&mut out, size);
            out.extend_from_slice(&data);
            out.extend_from_slice(&levels);
            for value in offsets {
                push_u32(&mut out, value);
            }
            for value in indices {
                out.extend_from_slice(&value.to_le_bytes());
            }
            assert_eq!(out, run("omm_compact", seed, &case).unwrap());
        }
    }

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
