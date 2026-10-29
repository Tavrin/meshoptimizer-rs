"""Regression checks for the narrow upstream NEON allowance."""
import os
import builtins
import contextlib
import io
import json
import tempfile
from pathlib import Path
from types import SimpleNamespace
import unittest
from unittest.mock import patch

with patch.dict(os.environ, DECODE_ART='.', CARGO_TARGET_DIR='.', MESHOPT_REFERENCE='.'):
    import differential
    from differential import neon_escape_group, allowed_cpp_divergence, stream

# One v1 record, stride four: one packed plane, three zero planes, 24-byte tail.
MINIMAL=bytes.fromhex('a1 a8 02 ffffffff')+bytes(16+24)

class EscapeClassTests(unittest.TestCase):
    def test_minimal_and_architecture_gate(self):
        self.assertEqual(len(MINIMAL),47)
        self.assertEqual(neon_escape_group(1,4,MINIMAL),dict(offset=3,block=0,plane=0,group=0))
        for machine in ('aarch64','arm64'):
            self.assertTrue(allowed_cpp_divergence(machine,1,4,MINIMAL))
        for machine in ('x86_64','riscv64','unknown'):
            self.assertFalse(allowed_cpp_divergence(machine,1,4,MINIMAL))

    def test_reject_other_widths_and_fifteen_escapes(self):
        for v,ctrl,sel in ((1,0,1),(0,0,2),(0,0,3),(1,2,0)):
            self.assertIsNone(neon_escape_group(1,4,stream(1,4,v,0,ctrl,sel)))
        changed=bytearray(MINIMAL);changed[3]=0xfe
        self.assertIsNone(neon_escape_group(1,4,changed))

    def test_reject_unreachable_or_unchecked_bytes(self):
        self.assertIsNone(neon_escape_group(0,4,MINIMAL))
        self.assertIsNone(neon_escape_group(1,4,MINIMAL[:26]))
        bad=bytearray(MINIMAL);bad[0]=0xa2
        self.assertIsNone(neon_escape_group(1,4,bad))
        # All-ff bytes in a literal plane are not a packed escape group.
        literal=bytes.fromhex('a1 ab')+bytes([255])*16+bytes(24)
        self.assertIsNone(neon_escape_group(16,4,literal))
        # A bad channel stops scalar parsing before the later escaped planes.
        src=bytes.fromhex('a1 aa a8 02 ffffffff')+bytes(16+24-10)+bytes(8)+bytes([3,0])
        self.assertIsNone(neon_escape_group(1,8,src))

    def test_mixed_group_consumption(self):
        for header,prefix,offset in ((0x09,bytes([255])*2+bytes(16),21),(0x0b,bytes([255])*8+bytes(16),27)):
            src=bytes([0xa1,0xa8,header])+prefix+bytes([255])*4+bytes(16+24)
            self.assertEqual(neon_escape_group(17,4,src)['offset'],offset)
        # Matching bytes beyond an incomplete first plane header are unreachable.
        src=bytes([0xa1,0xa8,0])+bytes(24)
        self.assertIsNone(neon_escape_group(257,4,src))

    def test_versions_controls_blocks_and_truncations(self):
        for v,ctrl,sel in ((0,0,1),(1,0,2),(1,1,1)):
            for n in (1,16,17,257):
                src=stream(n,4,v,0,ctrl,sel)
                self.assertIsNotNone(neon_escape_group(n,4,src))
                self.assertIsNotNone(neon_escape_group(n,4,src[:-1]))
        # The first block has no groups; the escape is in block two.
        src=bytes([0xa1,0xaa])+bytes.fromhex('a8 02 ffffffff')+bytes(16+24)
        self.assertEqual(neon_escape_group(257,4,src)['block'],256)

class DivergenceGateTests(unittest.TestCase):
    def test_injected_status_divergences_write_receipt_and_fail_closed(self):
        class Driver:
            def __init__(self,cmd):self.default='cpp-default' in str(cmd[0])
            def call(self,b):return (1 if self.default else 0).to_bytes(4,'little')+bytes(11)
            def close(self):pass
        class Random:
            def __init__(self,seed):pass
            def randrange(self,n):return 1
            def choice(self,values):return 4
        def ranges(*args):
            if args in ((34,),(64,81),(256,273)):return []
            if args==(2000,):return [1]
            return builtins.range(*args)
        outside=bytearray(MINIMAL);outside[3]=0xfe
        for machine,src,allowed in (('aarch64',MINIMAL,True),('arm64',MINIMAL,True),('aarch64',outside,False),('x86_64',MINIMAL,False)):
            with self.subTest(machine=machine,allowed=allowed), tempfile.TemporaryDirectory() as tmp:
                with patch.multiple(differential,Driver=Driver,art=Path(tmp),range=ranges,create=True), patch.object(differential.os,'uname',return_value=SimpleNamespace(machine=machine)), patch.object(differential.random,'Random',Random), patch.object(differential,'stream',return_value=src), patch.dict(os.environ,DECODE_NO_STD=''), contextlib.redirect_stdout(io.StringIO()):
                    if allowed:differential.main()
                    else:
                        with self.assertRaises(AssertionError):differential.main()
                receipt=json.loads((Path(tmp)/'differential.json').read_text())
                self.assertEqual(receipt['cpp_default_isa_allowed'],int(allowed))
                self.assertEqual(receipt['cpp_default_isa_failing'],int(not allowed))
                self.assertEqual(receipt['forced_level_differences'],0)

if __name__=='__main__':unittest.main()
