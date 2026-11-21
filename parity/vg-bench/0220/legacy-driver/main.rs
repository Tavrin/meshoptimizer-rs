mod before;mod after;
fn main(){
 let a=before::run();let b=after::run();assert_eq!(a,b,"legacy grids or MDAG 1 changed");println!("legacy grids and MDAG 1: {} bytes identical",a.len());
 let root=std::path::PathBuf::from(std::env::var("VG_EVIDENCE").unwrap());
 for name in ["bunny","dead_tree_trunk_02","namaqualand_boulder_02","coast_rocks_01","dragon"] {
  let path=root.join(format!("{name}.vgm"));let mut previous=None;
  reference_size(&root.join("final").join(format!("{name}.bevy.8T.bin")),name);
  for t in [1,8] {let a=before::corpus(&path,t);let b=after::corpus(&path,t);assert_eq!(a,b,"{name} {t}T legacy corpus changed");if let Some(p)=&previous{assert_eq!(p,&b,"{name} legacy worker identity");}println!("{name} {t}T legacy clusterlod fields and MDAG 1: {} bytes identical",a.len());previous=Some(b);}
 }
}

fn reference_size(path:&std::path::Path,name:&str) {
 use std::io::Read;
 let raw=std::fs::read(path).unwrap();let mut bytes=Vec::new();
 lz4_flex::frame::FrameDecoder::new(&raw[44..]).read_to_end(&mut bytes).unwrap();
 let mut at=0;let mut fields=Vec::new();
 for (key,size) in [("positions",4),("normals",4),("uvs",8),("indices",1),("nodes",400),("meshlets",32),("cull",48)] {
  let n=u64::from_le_bytes(bytes[at..at+8].try_into().unwrap()) as usize;at+=8+n*size;fields.push(format!("\"{key}\":{}",n*size));
 }
 assert_eq!(at,bytes.len());
 println!("{{\"kind\":\"reference_size\",\"mesh\":\"{name}\",{},\"length_headers\":56,\"total\":{}}}",fields.join(","),bytes.len());
}
