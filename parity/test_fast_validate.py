"""Focused fail-closed tests for disagreement acceptance and historical formats."""
import unittest
from parity import fast_validate as v


class NoiseAcceptanceTests(unittest.TestCase):
    def test_memory_disagreement_cannot_be_excused_by_clock_noise(self):
        old = {'memory_ratio': 1.1, 'ratio': 1.49, 'samples': [1.4, 1.45, 1.55, 1.6]}
        new = {'memory_ratio': 1.3, 'ratio': 1.49}
        self.assertFalse(v.within_full_noise('cache/test', old, '0.1', new))

    def test_timing_disagreement_requires_full_record_noise(self):
        old = {'memory_ratio': 1, 'ratio': 1.49, 'samples': [1.4, 1.45, 1.55, 1.6]}
        new = {'memory_ratio': 1, 'ratio': 1.51}
        self.assertTrue(v.within_full_noise('cache/test', old, '0.1', new))
        old['samples'] = [1.48] * 20
        self.assertFalse(v.within_full_noise('cache/test', old, '0.1', new))

    def test_both_decoder_gates_must_be_inside_full_noise(self):
        old = {'ratio': 1.49, 'minimum': 1, 'decoded_bytes': 1,
               'rust_samples': [.9] * 20, 'samples': [1.4, 1.45, 1.55, 1.6]}
        new = {'ratio': 1.51, 'samples_seconds': {'rust': [1.1] * 20}}
        self.assertFalse(v.within_full_noise('vertex/allocating/test', old, '0.2', new))
        old['rust_samples'] = [.9, .95, 1.05, 1.1]
        self.assertTrue(v.within_full_noise('vertex/allocating/test', old, '0.2', new))

    def test_nonraw_decoder_has_no_scalar_ratio_bar(self):
        old = {'ratio': 99, 'minimum': 1, 'decoded_bytes': 1,
               'rust_samples': [.9, .95, 1.05, 1.1], 'samples': [90, 95, 100, 110]}
        new = {'ratio': 100, 'samples_seconds': {'rust': [1.1] * 20}}
        self.assertTrue(v.within_full_noise('oct/allocating/test', old, '0.2', new))


class HistoricalRecordTests(unittest.TestCase):
    def test_source_export_prefix_is_normalized(self):
        root = '/example/artifacts/exports/current-main'
        snapshot = {root + '/src/meshlet.rs': 'library', root + '/Cargo.toml': 'build',
                    '/example/reference/src/clusterizer.cpp': 'oracle'}
        self.assertEqual(v.source_hashes({'sources': snapshot}, root), {'src/meshlet.rs': 'library'})
        self.assertEqual(v.canonical_snapshot(snapshot, root)['Cargo.toml'], 'build')

    def test_preprocessing_format_has_no_invented_wall_time(self):
        record = {'workloads': {'case': {'paired_ratio_stats': {'median': 1.2, 'raw_ratios': [1.2] * 10},
                                        'memory_ratio': 1, 'input': {'sha256': 'input'}}},
                  'families': {'family': {'passed': True}}, 'identities': {'source_sha256': {}}}
        rows, families, wall, _ = v.full_rows(record, '0.1.x')
        self.assertEqual(rows['case']['ratio'], 1.2)
        self.assertTrue(families['family'])
        self.assertIsNone(wall)


if __name__ == '__main__':
    unittest.main()
