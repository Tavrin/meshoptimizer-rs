use meshoptimizer_rs::{codec::*,Workspace};
mod fixtures;
#[no_mangle]
pub extern "C" fn check_vertices() -> u32 {
    let mut cases=0;
    for count in (0..=33).chain(64..=80).chain(256..=272) {
        for stride in [4,8,12,32,48,256] {
            for version in 0..=1 {
                let channels:Vec<u8>=if version==0 {vec![0]}else {[0,1,3].into_iter().chain((0..16).map(|r|r*16+2)).collect()};
                for channel in channels {
                    for control in 0..=if version==0 {0}else{3} {
                        for selector in 0..=if control>=2 {0}else{3} {
                            let source=fixtures::stream(count,stride,version,channel,control,selector);
                            for len in [source.len(),source.len()-1,source.len()/2,0] {
                                let run=|level|with_level(level,||{
                                    let mut w=Workspace::default();let mut out=vec![0xcc;count*stride+7];
                                    let result=decode_vertex_buffer_into(&mut out,count,stride,&source[..len],&mut w);
                                    (result,out,w.usage())
                                }).unwrap();
                                assert_eq!(run(Level::Scalar),run(Level::Wasm));cases+=1;
                            }
                        }
                    }
                }
            }
        }
    }
    cases
}
