"""Regressions for the coordinator's paired metric and hardware evidence."""
import unittest

import performance
import perf_profile


class Measurement(unittest.TestCase):
    def test_pair_median_is_not_ratio_of_backend_medians(self):
        result = performance.paired_stats({'rust': [1., 2., 10.], 'cpp': [2., 4., 1.]})
        self.assertEqual(result['raw_ratios'], [.5, .5, 10.])
        self.assertEqual(result['median'], .5)
        self.assertEqual(result['median_absolute_deviation'], 0.)
        self.assertGreater(result['coefficient_of_variation'], 0.)

    def test_incomplete_pairs_are_rejected(self):
        with self.assertRaises(ValueError):
            performance.paired_stats({'rust': [1., 2.], 'cpp': [1.]})

    def test_unsupported_counters_do_not_pass(self):
        with self.assertRaises(ValueError):
            perf_profile.counters('<not supported>,,cycles:u,0,0.00,,\n')

    def test_all_requested_counters_are_retained(self):
        text = ''.join(f'{n},,{name}:u,10000,100.00,,\n' for n,name in enumerate(
            ['cycles', 'instructions', 'branch-misses', 'cache-misses'], 1))
        result = perf_profile.counters(text)
        self.assertEqual(result['instructions']['count'], 2)
        self.assertEqual(result['cache-misses']['scheduled_percent'], 100.)


if __name__ == '__main__':
    unittest.main()
