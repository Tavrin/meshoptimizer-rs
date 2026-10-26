#!/usr/bin/env python3
"""Equalize benchmark output allocations; no algorithm changes."""
from pathlib import Path
import shutil
L=Path(__file__).resolve().parent
A=Path('/mnt/linux-extra/moss-scratch/meshopt-028-L1')
R=L.parents[1]
original=A/'original-adapter'
if not original.exists():
    shutil.copytree(L/'src',original/'src')
    for f in ['Cargo.toml','Cargo.lock']:shutil.copy2(L/f,original/f)
    p=original/'Cargo.toml';p.write_text(p.read_text().replace('path = "../.."',f'path = "{R}"'))
p=L/'src/main.rs';s=p.read_text()
a=s.index('            let r = match op {',s.index('"meshlets" | "meshlets-scan"'))
b=s.index('            Output::MeshOurs(r)',a)
s=s[:a]+'''            let bound = m::build_meshlets_bound(x.i.len(), 64,
                if op == "meshlets-flex" || op == "meshlets-spatial" {64} else {124}).unwrap();
            let mut r = m::Meshlets {
                meshlets: vec![m::Meshlet::default(); bound],
                vertices: vec![0; x.i.len().max(bound * 64)],
                triangles: vec![0; x.i.len().max(bound * 124 * 3)],
            };
            let (ms, v, t) = (&mut r.meshlets, &mut r.vertices, &mut r.triangles);
            let sizes = match op {
                "meshlets" => m::build_meshlets_into(ms,v,t,&x.i,p,vs,0.,w),
                "meshlets-scan" => m::build_meshlets_scan_into(ms,v,t,&x.i,n,vs,w),
                "meshlets-flex" => m::build_meshlets_flex_into(ms,v,t,&x.i,p,vs,64,0.,0.,w),
                _ => m::build_meshlets_spatial_into(ms,v,t,&x.i,p,vs,64,0.5,w),
            }.unwrap();
            r.meshlets.truncate(sizes.meshlets);
            r.vertices.truncate(sizes.vertices);
            r.triangles.truncate(sizes.triangles);
''' +s[b:]
s=s.replace('let mut v = vec![0; bound * 64];','let mut v = vec![0; ni.max(bound * 64)];').replace('let mut t = vec![0; bound * 124 * 3];','let mut t = vec![0; ni.max(bound * 124 * 3)];')
p.write_text(s)
p=L/'src/peer_x.rs';s=p.read_text().replace('vec![0;bound*64]','vec![0;x.i.len().max(bound*64)]').replace('vec![0;bound*124*3]','vec![0;x.i.len().max(bound*124*3)]');p.write_text(s)
