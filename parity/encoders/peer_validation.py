"""Explicit known 0.25/1.3 byte differences; no change to the 1.3 contract."""
import struct

def exp_separate_zero_fields(source, bits, cpp, peer):
    assert len(source) == len(cpp) == len(peer)
    differences = 0
    for raw, new, old in zip(struct.iter_unpack('<I', source), struct.iter_unpack('<I', cpp), struct.iter_unpack('<I', peer)):
        raw, new, old = raw[0], new[0], old[0]
        if new == old:
            continue
        # 0.25 resets zero to exponent zero; 1.3 inherits the last exponent.
        # Both mantissas must be exactly zero; nonzero components stay exact.
        assert raw & 0x7fffffff == 0
        assert old == (((-(bits - 1)) & 255) << 24)
        assert new & 0xffffff == 0
        differences += 1
    return differences
