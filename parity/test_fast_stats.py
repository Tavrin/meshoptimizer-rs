import math
import unittest
from unittest import mock

from parity import fast_stats
from parity import fast_qualify


class MedianSequenceTests(unittest.TestCase):
    def test_interval_is_unbounded_before_it_has_error_budget(self):
        self.assertEqual(fast_stats.median_interval([1.0] * 20, .05 / 624),
                         (0.0, math.inf))

    def test_exact_binomial_tail_covers_each_endpoint(self):
        alpha = .05 / 204
        for n in (25, 30, 60):
            low, high = fast_stats.median_interval(list(range(1, n + 1)), alpha)
            if math.isinf(high):
                continue
            k = int(low) - 1
            tail = sum(math.comb(n, j) for j in range(k + 1)) / 2**n
            self.assertLessEqual(2 * tail, alpha / (n * (n + 1)))

    def test_sequential_decision_waits_for_a_valid_interval(self):
        rust = [1.0] * 20
        cpp = [1.0] * 20
        self.assertFalse(fast_stats.stop(rust, cpp, .05 / 624)['stopped'])
        rust.extend([1.0] * 10)
        cpp.extend([1.0] * 10)
        verdict = fast_stats.stop(rust, cpp, .05 / 624)
        self.assertTrue(verdict['stopped'])
        self.assertEqual(verdict['reason'], 'clearly_below_case_bar')

    def test_separate_backend_medians_are_unbounded_at_first_look(self):
        decision = fast_stats.stop([1.0], [1.0], .05 / 132, paired=False)
        self.assertEqual((decision['lower'], decision['upper']), (0, math.inf))
        self.assertFalse(decision['stopped'])

    def test_codec_stops_on_only_applicable_bars(self):
        rust = [1.0] * 30
        cpp = [1.0] * 30
        alpha = .05 / 162
        for raw in (False, True):
            decision = fast_stats.stop(rust, cpp, alpha / 2, paired=False)
            decision['case_bar_class'] = 'unresolved'
            decision['reason'] = 'continue'
            decision = fast_stats.codec_stop(decision, rust, alpha / 2, .5, 1, raw)
            self.assertEqual(decision['registered_floor_class'], 'above_floor')
            self.assertEqual(decision['stopped'], not raw)
        decision = fast_stats.stop(rust, cpp, alpha / 2, paired=False)
        decision = fast_stats.codec_stop(decision, rust, alpha / 2, 1, 1, False)
        self.assertEqual(decision['registered_floor_class'], 'unresolved')
        self.assertFalse(decision['stopped'])

    def test_unresolved_floor_at_maximum_is_labelled_maximum(self):
        rust, cpp = [1.0] * 80, [1.0] * 80
        decision = fast_stats.stop(rust, cpp, .05 / 162, paired=False)
        decision = fast_stats.codec_stop(decision, rust, .05 / 324, 1, 1, True)
        self.assertTrue(decision['stopped'])
        self.assertEqual(decision['reason'], 'maximum')
        self.assertEqual(decision['registered_floor_class'], 'unresolved')


class CoreAdmissionTests(unittest.TestCase):
    def test_reserves_four_cores_and_rejects_busy_sibling(self):
        before = {cpu: (100, 50) for cpu in range(12)}
        after = {cpu: (200, 150 if cpu != 1 else 70) for cpu in range(12)}

        def topology(path):
            cpu = int(str(path).split('/cpu')[2].split('/')[0])
            field = path.name
            if field == 'physical_package_id':
                return '0'
            if field == 'core_id':
                return str(cpu // 2)
            return f'{cpu // 2 * 2}-{cpu // 2 * 2 + 1}'

        with (mock.patch.object(fast_stats, '_ticks', side_effect=[before, after]),
              mock.patch.object(fast_stats, '_frequency', return_value=2000000),
              mock.patch.object(fast_stats.time, 'sleep'),
              mock.patch.object(fast_stats.os, 'sched_getaffinity', return_value=set(range(12))),
              mock.patch.object(fast_stats.Path, 'read_text', topology)):
            result = fast_stats.physical_cores(seconds=0, requested=4)
        self.assertEqual(len(result['selected']), 2)
        self.assertEqual(result['reserved_physical_cores'], 4)
        self.assertNotIn(0, [c['core'] for c in result['selected']])


class TimingAdmissionTests(unittest.TestCase):
    def setUp(self):
        for name, value in [('GPU_LEASE', '/example/gpu-lease'),
                            ('SCOREBOARD_PATTERN', 'example-scoreboard-*')]:
            patch = mock.patch.object(fast_qualify, name, value)
            patch.start()
            self.addCleanup(patch.stop)

    @staticmethod
    def commands(command, **kwargs):
        if command == ['uptime']:
            return ' 02:30:00 load average: 11.99, 9.00, 8.00'
        if command[-1] == 'status':
            return 'GPU lease: FREE\n'
        return ''

    def test_blocks_before_deadline_even_when_load_is_low(self):
        with (mock.patch.object(fast_qualify.time, 'time', return_value=fast_qualify.TIMING_GATE_UNIX - 1),
              mock.patch.object(fast_qualify.subprocess, 'check_output',
                                side_effect=self.commands)):
            with self.assertRaises(SystemExit):
                fast_qualify.timing_admission()

    def test_deadline_and_load_both_required(self):
        with (mock.patch.object(fast_qualify.time, 'time', return_value=fast_qualify.TIMING_GATE_UNIX),
              mock.patch.object(fast_qualify.subprocess, 'check_output',
                                side_effect=lambda cmd, **kw: self.commands(cmd, **kw).replace('11.99', '12.00'))):
            with self.assertRaises(SystemExit):
                fast_qualify.timing_admission()
        with (mock.patch.object(fast_qualify.time, 'time', return_value=fast_qualify.TIMING_GATE_UNIX),
              mock.patch.object(fast_qualify.subprocess, 'check_output',
                                side_effect=self.commands)):
            self.assertEqual(fast_qualify.timing_admission()['load_one'], 11.99)

    def test_lease_and_active_scoreboard_each_block_low_load(self):
        for lease, units in [('GPU lease: HELD by test', ''),
                             ('GPU lease: UNKNOWN', ''),
                             ('GPU lease: FREE', 'example-scoreboard-test.service loaded active running test')]:
            def output(command, **kwargs):
                if command[-1] == 'status':
                    return lease
                if command[0] == 'systemctl':
                    return units
                return self.commands(command, **kwargs)
            with (self.subTest(lease=lease, units=units),
                  mock.patch.object(fast_qualify.time, 'time', return_value=fast_qualify.TIMING_GATE_UNIX),
                  mock.patch.object(fast_qualify.subprocess, 'check_output', side_effect=output)):
                with self.assertRaises(SystemExit):
                    fast_qualify.timing_admission()

    def test_unavailable_status_fails_closed(self):
        with (mock.patch.object(fast_qualify.time, 'time', return_value=fast_qualify.TIMING_GATE_UNIX),
              mock.patch.object(fast_qualify.subprocess, 'check_output', side_effect=OSError('missing'))):
            self.assertFalse(fast_qualify.timing_status()['allowed'])


if __name__ == '__main__':
    unittest.main()
