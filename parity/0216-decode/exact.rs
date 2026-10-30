//! Separately compiled base/candidate; result, Usage, whole destination.
mod fixtures;
#[derive(Debug,PartialEq)]
struct Observation { result: Result<Vec<u8>,String>, output: Vec<u8>, usage: (u64,usize) }
macro_rules! run {
    ($m:ident,$c:expr,$work:expr,$bytes:expr,$into:expr) => {{
        use $m as m;
        let (count,stride,source)= $c;
        let mut w=m::Workspace::new(m::Limits{max_work:$work,max_bytes:$bytes});
        let call=|w:&mut m::Workspace| {
            let mut output=vec![0xcc;count*stride+7];
            let result=if $into {
                m::codec::decode_vertex_buffer_into(&mut output,count,stride,source,w).map(|()|Vec::new())
            }else {m::codec::decode_vertex_buffer(count,stride,source,w)}.map_err(|e|format!("{e:?}"));
            let usage=w.usage();Observation{result,output,usage:(usage.work,usage.bytes)}
        };
        let cold=call(&mut w);
        let _=m::optimize_vertex_cache(&[0,1,2],3,&mut w);
        let warm=call(&mut w);
        w.set_limits(m::Limits{max_work:0,max_bytes:0});let _=call(&mut w);
        w.set_limits(m::Limits{max_work:$work,max_bytes:$bytes});
        [cold,warm,call(&mut w)]
    }}
}
fn main() {
    let mut comparisons=0u64;
    for count in 0..=17 {
        for stride in [4,12] {
            for version in 0..=1 {
                for channel in [0,1,2,0xf2,3] {
                    for control in 0..=if version==0 {0} else {3} {
                        let source=fixtures::stream(count,stride,version,channel,control,3);
                        for length in [0,source.len()/2,source.len()-1,source.len()] {
                            let c=(count,stride,&source[..length]);
                            let work=length+count*stride;
                            for into in [false,true] {
                                for limit in 0..=work+1 {
                                    assert_eq!(run!(baseline,c,limit as u64,usize::MAX,into),run!(candidate,c,limit as u64,usize::MAX,into),"{count}/{stride}/{version}/{channel}/{control}/{length}/{limit}/{into}");comparisons+=3;
                                }
                                for bytes in [0,1,count*stride/2,(count*stride).saturating_sub(1),count*stride,count*stride+1] {
                                    assert_eq!(run!(baseline,c,u64::MAX,bytes,into),run!(candidate,c,u64::MAX,bytes,into));comparisons+=3;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut seed=0x216u32;
    for i in 0..2000 {
        seed=seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let count=seed as usize%1025;
        let stride=[4,8,12,16,32,48,64,256][(seed>>16) as usize%8];
        let mut source=fixtures::stream(count,stride,(i%2) as u8,[0,1,2,0xf2][i%4],(i%4) as u8,(i%4) as u8);
        if i%3==0 {let at=seed as usize%source.len();source[at]^=0x81;}
        if i%5==0 {source.truncate(seed as usize%source.len());}
        let c=(count,stride,&source[..]);
        for into in [false,true] {
            for work in [u64::MAX,(source.len()+count*stride) as u64,(source.len()+count*stride).saturating_sub(1) as u64] {
                assert_eq!(run!(baseline,c,work,usize::MAX,into),run!(candidate,c,work,usize::MAX,into),"random/{i}");comparisons+=3;
            }
        }
    }
    println!("exact observations={comparisons} differences=0");
}
