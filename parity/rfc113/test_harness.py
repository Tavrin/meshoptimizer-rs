#!/usr/bin/env python3
"""Focused RFC 113 harness regressions."""
import sys
import tempfile
import unittest
from types import SimpleNamespace
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))
import codec
import run
import timing


class HarnessTests(unittest.TestCase):
    def test_moss_only_mismatch_is_a_failure(self):
        mismatch = run.first_mismatch(b'a' * 48, b'b' + b'a' * 47,
                                      b'a' * 48, 'reproduce')
        self.assertIsNotNone(mismatch)
        self.assertEqual(mismatch['side'], 'moss_cpp')
        self.assertEqual(mismatch['byte'], 0)

    def test_codec_creates_fresh_target_before_compiler(self):
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory) / 'fresh-target'
            calls = []
            def compiler(command, **_):
                self.assertTrue(target.is_dir())
                calls.append(command)
            with patch.object(run, 'TARGET', target), patch.object(codec.clod, 'TARGET', target), \
                 patch.object(codec.subprocess, 'run', side_effect=compiler):
                codec.build()
            self.assertEqual(len(calls), 2)
            self.assertEqual(calls[0][0], 'c++')


class TimingTests(unittest.TestCase):
    def test_admission_distinguishes_sleep_from_measurement(self):
        for lease, child, admitted in (
            ('GPU lease: FREE', 'sleep', True),
            ('GPU lease: FREE', 'perf', False),
            ('GPU lease: HELD', 'sleep', False),
        ):
            def command(args, **_):
                if args[-1] == 'status':
                    out = lease
                elif 'list-units' in args:
                    out = 'moss-scoreboard-test.service loaded active running'
                else:
                    out = '100'
                return SimpleNamespace(returncode=0, stdout=out)
            def process(pid):
                return dict(pid=pid, name='bash' if pid == 100 else child,
                            children=[101] if pid == 100 else [])
            with patch.object(timing.subprocess, 'run', side_effect=command), \
                 patch.object(timing, 'proc', side_effect=process), \
                 patch.object(timing, 'record'):
                self.assertEqual(timing.gate(), admitted)

    def test_stopping_interval_keeps_borderline_cases(self):
        for ratios, verdict in (([1.1]*5, 'PASS'), ([2.0]*5, 'FAIL'),
                                ([1.3, 1.6, 1.4, 1.7, 1.5], 'BORDERLINE')):
            pairs = [dict(cpp=1., rust=r) for r in ratios]
            self.assertEqual(timing.interval(pairs)['verdict'], verdict)


if __name__ == '__main__':
    unittest.main()
