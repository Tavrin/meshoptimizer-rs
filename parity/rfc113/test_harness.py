#!/usr/bin/env python3
"""Focused RFC 113 harness regressions."""
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))
import codec
import run


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


if __name__ == '__main__':
    unittest.main()
