"""Regression checks for scalar oracle assertions and ISA disclosures."""
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

SUCCESS=bytes(15)
FAILURE=(1).to_bytes(4,'little')+bytes(11)
MINIMAL=bytes.fromhex('a1 a8 02 ffffffff')+bytes(40)

class DivergenceGateTests(unittest.TestCase):
    def run_case(self,machine,outputs,rejected=False,systematic=False,src=MINIMAL):
        class Driver:
            def __init__(self,cmd):
                self.arm=next((k for k in ('cpp-default','cpp-scalar') if k in str(cmd[0])),str(cmd[-1]))
            def call(self,b):return outputs.get(self.arm,outputs.get('scalar',SUCCESS))
            def close(self):pass
        class Random:
            def __init__(self,seed):pass
            def randrange(self,n):return 1
            def choice(self,values):return 4
        def ranges(*args):
            if args==(34,):return [1] if systematic else []
            if args in ((64,81),(256,273)):return []
            if args==(2000,):return [] if systematic else [1]
            return builtins.range(*args)
        with tempfile.TemporaryDirectory() as tmp:
            levels=['scalar','neon'] if machine in ('aarch64','arm64') else ['scalar','sse2','ssse3','sse41']
            with patch.multiple(differential,Driver=Driver,art=Path(tmp),levels=levels,range=ranges,create=True), patch.object(differential.os,'uname',return_value=SimpleNamespace(machine=machine)), patch.object(differential.random,'Random',Random), patch.object(differential,'stream',return_value=src), patch.dict(os.environ,DECODE_NO_STD=''), contextlib.redirect_stdout(io.StringIO()):
                if rejected:
                    with self.assertRaises(AssertionError):differential.main()
                else:differential.main()
            receipt=Path(tmp)/'differential.json'
            return json.loads(receipt.read_text()) if receipt.exists() else None

    def test_arm64_status_splits_disclosed_without_classification(self):
        for machine in ('aarch64','arm64'):
            for src in (MINIMAL,b'\xa0'+bytes(40)):
                for systematic in (False,True):
                    with self.subTest(machine=machine,src=src,systematic=systematic):
                        r=self.run_case(machine,{'cpp-default':FAILURE},src=src,systematic=systematic)
                        self.assertEqual(r['cpp_default_isa_divergences'],r['cases'])
                        self.assertTrue(r['cpp_default_isa_disclosure_only'])
                        self.assertEqual(r['cpp_references'],['cpp-scalar'])
                        self.assertEqual(r['isa_first'][0]['scalar_status'],'00000000')
                        self.assertEqual(r['isa_first'][0]['default_status'],'01000000')
                        self.assertLessEqual(len(r['isa_first']),4)

    def test_arm64_reverse_status_split_disclosed(self):
        r=self.run_case('arm64',{'scalar':FAILURE,'cpp-default':SUCCESS})
        self.assertEqual(r['cpp_default_isa_divergences'],1)
        self.assertEqual(r['isa_first'][0]['scalar_status'],'01000000')
        self.assertEqual(r['isa_first'][0]['default_status'],'00000000')

    def test_arm64_success_bytes_disclosed(self):
        r=self.run_case('arm64',{'cpp-default':SUCCESS[:-1]+b'\x01'})
        self.assertEqual(r['cpp_default_isa_divergences'],1)
        self.assertEqual(r['cpp_success_differences'],0)

    def test_arm64_default_failure_writes_are_not_reference(self):
        r=self.run_case('aarch64',{'scalar':FAILURE,'cpp-default':FAILURE[:-1]+b'\x01'})
        self.assertEqual(r['cpp_default_isa_divergences'],0)
        self.assertEqual(r['cpp_failure_write_differences'],0)

    def test_scalar_failure_writes_remain_disclosure(self):
        for machine in ('arm64','x86_64'):
            r=self.run_case(machine,{'scalar':FAILURE,'cpp-scalar':FAILURE[:-1]+b'\x01'})
            self.assertEqual(r['cpp_failure_write_differences'],1)
            self.assertEqual(r['cpp_default_isa_divergences'],0)

    def test_scalar_status_and_success_bytes_always_asserted(self):
        for machine in ('arm64','aarch64','x86_64'):
            for bad in (FAILURE,SUCCESS[:-1]+b'\x01'):
                for systematic in (False,True):
                    with self.subTest(machine=machine,bad=bad,systematic=systematic):
                        self.run_case(machine,{'cpp-scalar':bad},rejected=True,systematic=systematic)

    def test_rust_neon_status_and_failure_writes_always_asserted(self):
        for outputs in ({'neon':FAILURE},{'scalar':FAILURE,'neon':FAILURE[:-1]+b'\x01'},{'neon':SUCCESS[:-1]+b'\x01'}):
            self.run_case('aarch64',outputs,rejected=True)

    def test_x86_divergences_fail_after_receipt(self):
        for bad in (FAILURE,SUCCESS[:-1]+b'\x01'):
            for systematic in (False,True):
                r=self.run_case('x86_64',{'cpp-default':bad},rejected=True,systematic=systematic)
                self.assertEqual(r['cpp_default_isa_divergences'],r['cases'])
                self.assertFalse(r['cpp_default_isa_disclosure_only'])
                self.assertEqual(r['cpp_references'],['cpp-scalar','cpp-default'])

if __name__=='__main__':unittest.main()
