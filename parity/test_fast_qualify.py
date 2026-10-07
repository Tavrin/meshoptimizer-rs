"""Untimed regression checks for qualification identity and worker plumbing."""
import copy
import json
import os
from pathlib import Path
import struct
import sys
import tempfile
import types
import unittest
from unittest import mock

from parity import fast_qualify as q, fast_validate as v, run_fast_validation as driver


def record():
    rust, scalar = [1.0] * 30, [1.0] * 30
    decision = q.fs.stop(rust, scalar, .05)
    row = {'case': 'cache/test', 'family': 'cache', 'input_sha256': 'input',
           'samples_seconds': {'rust': rust, 'scalar': scalar},
           'ratio': 1.0, 'memory_ratio': 1.0, 'sequential': decision,
           'case_pass': True, 'reused': False}
    return {'schema': 'meshopt-fast-qualify/1', 'phase': '0.1', 'profile': 'moss',
            'complete': True, 'source_unchanged': True, 'binary_unchanged': True,
            'smoke': False, 'full': True, 'wall_seconds': 1,
            'source_root': '/example/source',
            'harness_sha256': q.sha(q.ROOT / 'parity/fast_qualify.py'),
            'statistics_sha256': q.sha(q.ROOT / 'parity/fast_stats.py'),
            'bar': {'family_geometric_mean': 1.25, 'case_maximum': 1.5, 'memory': 1.25},
            'sequential_error': {'maximum_pairs': 80, 'per_case_alpha': .05},
            'identity': {'source_sha256': {'rust_and_harness': {'src/lib.rs': 'library'},
                                         'upstream': {'src/file.cpp': 'oracle'},
                                         'dependencies': {'dep/file': 'dependency'}},
                         'effective_profile_overrides': {}},
            'rows': [row], 'families': {'cache': q.family_verdict([row], '0.1')}}


class RecordTests(unittest.TestCase):
    def test_complete_record_compares_without_timing(self):
        fast = record()
        full = {'workloads': {'cache/test': {'rust_cpp_ratio': 1,
                    'paired_ratio_stats': {'median': 1, 'raw_ratios': [1.0] * 20},
                    'memory_ratio': 1, 'input': {'sha256': 'input'}}},
                'families': {'cache': {'passed': True}}, 'identities': fast['identity'],
                'started_unix': 1, 'finished_unix': 3}
        with tempfile.TemporaryDirectory() as directory:
            fast_path, full_path = (Path(directory) / name for name in ('fast.json', 'full.json'))
            q.write(fast_path, fast)
            q.write(full_path, full)
            result = v.compare({'fast': str(fast_path), 'full': str(full_path),
                                'phase': '0.1', 'profile': 'moss', 'revision': 'main'})
        self.assertEqual((result['case_agreements'], result['family_agreements']), (1, 1))

    def test_stored_verdict_and_duplicates_cannot_replace_raw_evidence(self):
        v.verify_fast_record(record())
        for change in ('ratio', 'case_pass', 'family', 'duplicate', 'bar', 'partial'):
            damaged = record()
            row = damaged['rows'][0]
            if change == 'ratio':
                row['ratio'] = 1.6
            elif change == 'case_pass':
                row['case_pass'] = False
            elif change == 'family':
                damaged['families']['cache']['pass'] = False
            elif change == 'duplicate':
                damaged['rows'].append(copy.deepcopy(row))
            elif change == 'bar':
                damaged['bar']['case_maximum'] = 2
            else:
                row['samples_seconds']['rust'] = [1.0]
            with self.subTest(change=change), self.assertRaises(ValueError):
                v.verify_fast_record(damaged)

    def test_self_declared_release_grade_is_not_reusable(self):
        old = record()
        old['release_grade'] = True
        self.assertFalse(q.prior_is_qualified(Path('missing'), old, None))
        self.assertFalse(q.prior_is_qualified(Path('missing'), old, {'maximum_pairs': 20}))
        with mock.patch.object(q, 'sha', return_value='record-digest'):
            self.assertFalse(q.prior_is_qualified(Path('example'), old,
                             {'maximum_pairs': 80, 'validated_records_sha256': []}))
            self.assertTrue(q.prior_is_qualified(Path('example'), old,
                            {'maximum_pairs': 80, 'validated_records_sha256': ['record-digest']}))
            old['source_unchanged'] = False
            self.assertFalse(q.prior_is_qualified(Path('example'), old,
                             {'maximum_pairs': 80, 'validated_records_sha256': ['record-digest']}))

    def test_rejected_manifest_revokes_receipt_and_returns_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'parity').mkdir()
            manifest = root / 'manifest.json'
            q.write(manifest, {'comparisons': [], 'smoke_noise': []})
            q.write(root / 'validation-receipt.json', {'accepted': True})
            with (mock.patch.object(v, 'ROOT', root), mock.patch.object(v, 'ART', root),
                  mock.patch.object(sys, 'argv', ['fast_validate.py', str(manifest)]),
                  self.assertRaises(SystemExit) as error):
                v.main()
            self.assertEqual(error.exception.code, 1)
            self.assertFalse((root / 'validation-receipt.json').exists())
            self.assertFalse(json.loads((root / 'validation-results.json').read_text())['accepted'])


class IdentityTests(unittest.TestCase):
    def test_export_inside_artifacts_is_allowed_but_target_ancestor_is_not(self):
        env = {'CARGO_TARGET_DIR': '/example/target', 'MESHOPT_ARTIFACTS': '/example/artifacts',
               'MESHOPT_REFERENCE': '/example/reference'}
        with (mock.patch.dict(os.environ, env),
              mock.patch.object(q, 'ROOT', Path('/example/artifacts/exports/source')),
              mock.patch.object(q, 'ART', Path('/example/artifacts')),
              mock.patch.object(q, 'REFERENCE', Path('/example/reference')),
              mock.patch.object(q, 'TARGET', Path('/example/target'))):
            q.require_paths()
            with mock.patch.object(q, 'TARGET', Path('/example')):
                with self.assertRaises(ValueError):
                    q.require_paths()

    def test_nested_harness_and_oracle_edits_change_fingerprint(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / 'source'
            reference = Path(directory) / 'reference'
            (root / 'parity/p03').mkdir(parents=True)
            (reference / 'demo').mkdir(parents=True)
            (root / 'Cargo.toml').write_text('manifest')
            (root / 'Cargo.lock').write_text('lock')
            nested = root / 'parity/p03/Cargo.toml'
            oracle = reference / 'demo/clusterlod.h'
            nested.write_text('build')
            oracle.write_text('oracle')
            with mock.patch.object(q, 'ROOT', root), mock.patch.object(q, 'REFERENCE', reference):
                first = q.fingerprint('0.3', 'consumer', {}, 'case')
                nested.write_text('changed build')
                second = q.fingerprint('0.3', 'consumer', {}, 'case')
                oracle.write_text('changed oracle')
                third = q.fingerprint('0.3', 'consumer', {}, 'case')
            self.assertNotEqual(first, second)
            self.assertNotEqual(second, third)

    def test_post_run_snapshot_reads_live_sources(self):
        fake = types.SimpleNamespace(snapshot=mock.Mock(return_value={'src': 'old'}))
        with (mock.patch.object(q, 'module', return_value=fake),
              mock.patch.object(q, 'dependency_files', return_value={'dep': 'old'}) as dependencies):
            self.assertTrue(q.source_unchanged('0.3', {'sources': {'src': 'old'}, 'dependencies': {'dep': 'old'}}))
            dependencies.return_value = {'dep': 'new'}
            self.assertFalse(q.source_unchanged('0.3', {'sources': {'src': 'old'}, 'dependencies': {'dep': 'old'}}))
            fake.snapshot.return_value = {'src': 'new'}
            self.assertFalse(q.source_unchanged('0.3', {'sources': {'src': 'old'}, 'dependencies': {'dep': 'new'}}))

    def test_target_cleanup_requires_ownership(self):
        with tempfile.TemporaryDirectory() as directory:
            target, artifacts = (Path(directory) / name for name in ('target', 'artifacts'))
            target.mkdir()
            valuable = target / 'existing'
            valuable.write_text('keep')
            with mock.patch.object(driver, 'TARGET', target), mock.patch.object(driver, 'ART', artifacts):
                with self.assertRaises(ValueError):
                    driver.claim_target()
                with self.assertRaises(ValueError):
                    driver.cleanup_target()
                self.assertTrue(valuable.exists())
                valuable.unlink()
                driver.claim_target()
                driver.cleanup_target()
                self.assertFalse(target.exists())

    def test_comparison_cache_binds_referenced_records(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'parity').mkdir()
            (root / 'parity/run_fast_validation.py').write_text('driver')
            (root / 'parity/fast_qualify.py').write_text('runner')
            (root / 'parity/fast_stats.py').write_text('stats')
            data = root / 'fast.json'
            manifest = root / 'manifest.json'
            q.write(data, {'version': 1})
            q.write(manifest, {'comparisons': [{'fast': str(data)}]})
            journal = {}
            process = mock.Mock(returncode=0)
            with (mock.patch.object(driver, 'ROOT', root), mock.patch.object(driver, 'ART', root),
                  mock.patch.object(driver, 'TARGET', root / 'target'),
                  mock.patch.object(driver.subprocess, 'run', return_value=process) as run):
                driver.run_attempt('compare', ['python', str(manifest)], root, root / 'results', False, journal)
                driver.run_attempt('compare', ['python', str(manifest)], root, root / 'results', False, journal)
                self.assertEqual(run.call_count, 1)
                q.write(data, {'version': 2})
                driver.run_attempt('compare', ['python', str(manifest)], root, root / 'results', False, journal)
                self.assertEqual(run.call_count, 2)


class WorkerTests(unittest.TestCase):
    def test_codec_worker_preserves_alternating_pairs_and_scalar_output(self):
        order = []

        class Driver:
            def __init__(self, name):
                self.name = name

            def call(self, data):
                samples = struct.unpack_from('<I', data, 32)[0]
                if samples:
                    order.append(self.name)
                return 0, b'output', [.04] if samples else []

            def close(self):
                pass

        core = {'cpu': 0}
        with tempfile.TemporaryDirectory() as directory:
            task = q.case_file(Path(directory), 'bounds/allocating/test', bytes(44), 'bounds/allocating')
            with (mock.patch.object(q, 'module', return_value=types.SimpleNamespace(Driver=Driver)),
                  mock.patch.object(q.fs, 'core_load', return_value={}),
                  mock.patch.object(q.fs, 'add_core_delta', return_value={}),
                  mock.patch.dict(os.environ, {'MESHOPT_FAST_CORE': json.dumps(core)})):
                row = q.measure_task('0.4', task, {key: key for key in ('rust', 'scalar', 'simd')}, .05, 80)
        self.assertTrue(row['case_pass'])
        self.assertEqual(order[1:7], ['rust', 'scalar', 'simd', 'simd', 'scalar', 'rust'])
        self.assertEqual(len(row['samples_seconds']['rust']), row['sequential']['pairs'])


if __name__ == '__main__':
    unittest.main()
