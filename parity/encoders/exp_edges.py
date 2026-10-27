#!/usr/bin/env python3
"""Exact Exp parity over finite exponent classes, precision and carry state."""
import json
import os
from pathlib import Path
import struct
import run as lane

assert os.environ.get('PARITY_RUNNER_ACTIVE')
A = lane.ART
profile = os.environ['MESHOPT_EDGE_PROFILE']
ds = {'rust': lane.Driver(A / f'codec-driver-{profile}'),
      'cpp': lane.Driver(A / 'cpp-scalar')}
records = []
try:
    for lower, upper in ((0, 253), (0, 254), (26, 252)):
        words = [sign | exponent << 23 | mantissa
                 for exponent in range(lower, upper + 1) for sign in (0, 0x80000000)
                 for mantissa in (0, 1, 0x3fffff, 0x7ffffe, 0x7fffff)]
        # Interleave zero to test the inherited per-component exponent.
        words = [value for word in words for value in (word, 0, word ^ 0x80000000, 0x80000000)]
        for stride in (4, 8, 12, 16, 32, 256):
            width = stride // 4
            source = struct.pack('<' + 'I' * (len(words) // width * width), *words[:len(words) // width * width])
            count = len(source) // stride
            for bits in range(1, 25):
                for mode in range(4):
                    for into in (False, True):
                        request = lane.request(16, count, stride, source, mode=mode | (128 if into else 0), level=bits)
                        results = {key: driver.call(request)[:2] for key, driver in ds.items()}
                        expected = results['cpp']
                        assert (results['rust'][0] == 0) == (expected[0] == 0), (upper, stride, bits, mode, into)
                        if expected[0] == 0:
                            assert results['rust'] == expected, (upper, stride, bits, mode, into)
                        records.append({'lower_exponent_field': lower, 'upper_exponent_field': upper, 'stride': stride, 'bits': bits, 'mode': mode, 'into': into, 'success': expected[0] == 0})
finally:
    for driver in ds.values():
        driver.close()
lane.save('exp-edges-' + profile, {'cases': records, 'mismatches': 0, 'binaries': {'rust': lane.sha(A / f'codec-driver-{profile}'), 'cpp': lane.sha(A / 'cpp-scalar')}})
print('Exp exponent classes PASS', profile, len(records))
