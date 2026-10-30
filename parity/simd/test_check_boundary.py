"""The retained oracle exemption covers only its pinned bytes and exact path."""
import contextlib
import hashlib
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import check_boundary


class RetainedOracleBoundaryTests(unittest.TestCase):
    def test_only_pinned_oracle_is_exempt(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            module = root / 'src/codec/simd'
            module.mkdir(parents=True)
            (module / 'mod.rs').write_text('#![allow(unsafe_code)]\n')
            (module / 'SAFETY.md').write_text('')
            oracle = root / 'parity/retained/peer-driver.rs'
            oracle.parent.mkdir(parents=True)
            original = 'unsafe extern "C" fn oracle() {}\n'
            oracle.write_text(original)
            digest = hashlib.sha256(oracle.read_bytes()).hexdigest()
            with patch.multiple(check_boundary, ROOT=root, MODULE=module,
                                PEER_FFI_DRIVERS=set(),
                                RETAINED_PEER_FFI_DRIVERS={oracle: digest}):
                with contextlib.redirect_stdout(io.StringIO()):
                    check_boundary.main()

                oracle.write_text(original + 'unsafe fn added() {}\n')
                with self.assertRaisesRegex(AssertionError, 'retained peer driver changed'):
                    check_boundary.main()

                oracle.write_text(original)
                unlisted = oracle.with_name('other.rs')
                unlisted.write_text(original)
                with self.assertRaisesRegex(AssertionError, 'unsafe outside audited module'):
                    check_boundary.main()

                unlisted.unlink()
                (module / 'extra.rs').write_text('unsafe fn added() {}\n')
                with self.assertRaisesRegex(AssertionError, 'only unsafe blocks permitted'):
                    check_boundary.main()


if __name__ == '__main__':
    unittest.main()
