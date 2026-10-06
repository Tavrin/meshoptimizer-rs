"""Scoped timing must preserve frozen coverage and the existing D146 policy."""
import subprocess
import sys
import unittest
from pathlib import Path

from benchmark import FAMILIES, early_screen, stage2_screen, scoped_inventory, scope_complete


class LeanTests(unittest.TestCase):
    def test_touched_inventory_includes_all_shapes_and_caller_forms(self):
        requested = {'remesh', 'overdraw', 'coverage', 'omm_compact'}
        inventory = scoped_inventory(requested)
        self.assertEqual(len(inventory), 38)
        rows = [dict(family=f, api=a, shape=s) for f, a, s in inventory]
        self.assertTrue(scope_complete(rows, requested))
        self.assertFalse(scope_complete(rows[:-1], requested))
        self.assertFalse(scope_complete(rows[:-1] + [rows[0]], requested))
        self.assertFalse(scope_complete([], set()))

    def test_subset_cannot_establish_full_matrix_coverage(self):
        inventory = scoped_inventory({'remesh'})
        rows = [dict(family=f, api=a, shape=s) for f, a, s in inventory]
        self.assertFalse(scope_complete(rows, set(FAMILIES)))
        self.assertEqual(len(scoped_inventory(set(FAMILIES))), 160)

    def test_all_stage_one_looks_keep_maximum_bar(self):
        for count in [5, 10, 20]:
            self.assertEqual(early_screen([{'ratio': 1.0}] * count)['verdict'], 'PASS')
            self.assertEqual(early_screen([{'ratio': 1.7}] * count)['verdict'], 'FAIL')
            borderline = [{'ratio': 1.4 if i % 2 else 1.6} for i in range(count)]
            self.assertEqual(early_screen(borderline)['verdict'], 'BORDERLINE')

    def test_stage_two_requires_thirty_fresh_pairs(self):
        with self.assertRaises(ValueError):
            stage2_screen([{'ratio': 1.0}] * 20)
        self.assertEqual(stage2_screen([{'ratio': 1.0}] * 30)['verdict'], 'PASS')
        self.assertEqual(stage2_screen([{'ratio': 1.7}] * 30)['verdict'], 'FAIL')
        fresh = [{'ratio': 1.4 if i % 2 else 1.6} for i in range(30)]
        self.assertEqual(stage2_screen(fresh)['verdict'], 'INCONCLUSIVE')

    def test_invalid_scope_options_reject_before_build_or_admission(self):
        script = Path(__file__).with_name('benchmark.py')
        for options, message in [
            (['--families', 'all', '--mode', 'lean'], 'explicit touched-family subset'),
            (['--families', 'remesh', '--mode', 'final'], 'final requires every family'),
            (['--families', 'remesh', '--mode', 'lean', '--cases', 'remesh:0'], 'do not permit case filtering'),
        ]:
            result = subprocess.run([sys.executable, str(script), '--consumer-profile', 'moss', *options],
                                    text=True, capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(message, result.stderr)
            self.assertNotIn('Compiling', result.stderr)


if __name__ == '__main__':
    unittest.main()
