"""Admission and pinning regression tests; synthetic clocks avoid real waits."""
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import quiet


class Admission(unittest.TestCase):
    def run_wait(self, loads, clocks):
        record = {}
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'wait.json'
            with patch.object(quiet, 'load', side_effect=loads), \
                 patch.object(quiet.time, 'monotonic', side_effect=clocks), \
                 patch.object(quiet.time, 'sleep') as sleep:
                try:
                    quiet.wait(record, path, 'test')
                except quiet.QuietTimeout:
                    return record, sleep.call_args_list, True, path.read_text()
                return record, sleep.call_args_list, False, path.read_text()

    def test_strict_threshold_and_wait_record(self):
        record, sleeps, timeout, persisted = self.run_wait(
            [[1.5, 2., 3.], [1.49, 2., 3.]], [100., 100., 130.])
        self.assertFalse(timeout)
        admission = record['quiet_waits'][0]
        self.assertTrue(admission['passed'])
        self.assertEqual(admission['waited_seconds'], 30.)
        self.assertEqual(len(admission['checks']), 2)
        self.assertEqual(sleeps[0].args, (30,))
        self.assertIn('1.49', persisted)

    def test_timeout_fails_closed_and_retains_last_check(self):
        record, sleeps, timeout, persisted = self.run_wait(
            [[2., 2., 2.], [1.5, 2., 2.]], [100., 100., 3700.])
        self.assertTrue(timeout)
        admission = record['quiet_waits'][0]
        self.assertFalse(admission['passed'])
        self.assertEqual(admission['waited_seconds'], 3600.)
        self.assertEqual(len(sleeps), 1)
        self.assertIn('3600.0', persisted)

    def test_immediate_admission_does_not_sleep(self):
        record, sleeps, timeout, _ = self.run_wait([[0.2, 4., 4.]], [100., 100.])
        self.assertFalse(timeout)
        self.assertEqual(sleeps, [])
        self.assertTrue(record['quiet_waits'][0]['passed'])


if __name__ == '__main__':
    unittest.main()
