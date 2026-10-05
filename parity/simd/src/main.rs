//! Native arithmetic release gate entry point; ranges are separately retained.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("edges") {
        assert_eq!(meshopt_simd_qualification::check_edges(), 0);
        println!("{{\"family\":\"edges\",\"mismatches\":0}}");
        return;
    }
    let family = args.get(1).expect("sqrt|exp|oct8|oct16|quat");
    let f = match family.as_str() {
        "sqrt" => 0,
        "exp" => 1,
        "oct8" => 2,
        "oct16" => 3,
        "quat" => 4,
        _ => panic!("invalid family"),
    };
    let start: u64 = args.get(2).expect("start").parse().unwrap();
    let end: u64 = args.get(3).expect("end").parse().unwrap();
    assert!(start <= end && end <= 1u64 << 32 && end - start <= u64::from(u32::MAX));
    let failure = meshopt_simd_qualification::check_range(f, start as u32, (end - start) as u32);
    assert_eq!(failure, 0, "{family} mismatch index plus one {failure}");
    println!("{{\"family\":\"{family}\",\"start\":{start},\"end\":{end},\"records\":{},\"mismatches\":0}}",end-start);
}
