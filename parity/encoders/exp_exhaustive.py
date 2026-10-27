#!/usr/bin/env python3
"""All normal mantissas and both signs vs scalar C++ 1.3, precisions 1..22.

One normal exponent represents the exact power-of-two scaling equivalence
class. exp_edges.py separately covers the exponent-domain guards and fallback.
This is untimed correctness evidence; invoke through CPU admission.
"""
import array
import hashlib
import os
import sys
import run as lane

assert os.environ.get('PARITY_RUNNER_ACTIVE')
profile = sys.argv[1]
A = lane.ART
source = array.array('I', (sign | 0x3f800000 | mantissa
                          for sign in (0, 0x80000000) for mantissa in range(1 << 23)))
if sys.byteorder != 'little':
    source.byteswap()
raw = source.tobytes()
del source
cpp = lane.Driver(A / 'cpp-scalar')
rust = lane.Driver(A / f'codec-driver-{profile}')
rows = []
try:
    for bits in range(1, 23):
        request = lane.request(16, len(raw) // 16, 16, raw, level=bits)
        expected = cpp.call(request)[:2]
        actual = rust.call(request)[:2]
        assert expected[0] == actual[0] == 0
        assert actual == expected, bits
        rows.append({'bits': bits, 'output_sha256': hashlib.sha256(expected[1]).hexdigest()})
        print('mantissas', profile, bits, 'PASS', flush=True)
finally:
    cpp.close()
    rust.close()
lane.save('exp-exhaustive-' + profile, {'profile': profile, 'mantissas': 1 << 23, 'signs': 2,
          'comparisons': (1 << 24) * 22, 'precision_cases': rows, 'mismatches': 0,
          'input_sha256': hashlib.sha256(raw).hexdigest(),
          'binaries': {key: lane.sha(A / name) for key, name in {'rust': f'codec-driver-{profile}', 'cpp': 'cpp-scalar'}.items()}})
