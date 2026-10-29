// Hand-authored wire streams exercise channels independently of encoder choices.
pub fn stream(count: usize, stride: usize, version: u8, channel: u8, control: u8, selector: u8) -> Vec<u8> {
    let mut data = vec![0xa0 | version];
    let block_size = ((8192 / stride) & !15).min(256);
    let mut offset = 0;
    while offset < count {
        let block = block_size.min(count - offset);
        let groups = block.div_ceil(16);
        if version == 1 { data.extend(vec![control.wrapping_mul(85); stride / 4]); }
        for _ in 0..stride {
            if version == 1 && control == 2 { continue; }
            if version == 1 && control == 3 {
                data.extend((0..block).map(|i| (i * 73 + offset * 3 + 17) as u8));
                continue;
            }
            data.extend(vec![selector.wrapping_mul(85); groups.div_ceil(4)]);
            let bits = if version == 0 { [0,2,4,8][selector as usize] }
                else if control == 0 { [0,1,2,4][selector as usize] }
                else { [1,2,4,8][selector as usize] };
            for _ in 0..groups {
                if bits == 8 { data.extend([0x93;16]); }
                else if bits != 0 {
                    // Every field escapes, including the final packed field.
                    data.extend(vec![255; bits as usize * 2]);
                    data.extend((0..16).map(|i| (i * 71 + 19) as u8));
                }
            }
        }
        offset += block;
    }
    let tail = stride + if version == 0 {0} else {stride/4};
    let padded = tail.max(if version == 0 {32} else {24});
    data.extend(vec![0; padded-tail]);
    data.extend((0..stride).map(|i| (i*37+251) as u8));
    if version == 1 { data.extend(vec![channel; stride/4]); }
    data
}
