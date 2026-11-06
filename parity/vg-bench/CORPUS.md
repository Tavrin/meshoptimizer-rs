# Benchmark corpus

Used locally and never redistributed. `fetch_corpus.sh` downloads each file from
its publisher into a directory outside the repository and checks it against
`corpus.sha256`. Meshes are read as data only.

| Name | Source | Licence | Notes |
| --- | --- | --- | --- |
| bunny | Stanford 3D Scanning Repository, `bunny/reconstruction/bun_zipper.ply` | Research use; credit the repository; no commercial use of the models or images without permission | 69,451 triangles; the mesh is also the Bevy meshlet example mesh |
| dragon | Stanford 3D Scanning Repository, `dragon_recon/dragon_vrip.ply` | same | about 871k triangles |
| lucy | Stanford 3D Scanning Repository, `lucy.ply` | same | about 28M triangles |
| namaqualand_boulder_02 | Poly Haven, 1k glTF buffer only | CC0 1.0 (all Poly Haven assets) | scanned boulder |
| coast_rocks_01 | Poly Haven, 1k glTF buffer only | CC0 1.0 | scanned rock cluster |
| dead_tree_trunk_02 | Poly Haven, 1k glTF buffer only | CC0 1.0 | scanned trunk |
| flighthelmet | Khronos glTF Sample Assets, `Models/FlightHelmet` | Model files CC0 1.0 (the asset's `LICENSE.md`, copied next to the data); conversion credit: Gary Hsu | six materials; fetched for the later attribute and material work, not used by the MDAG 1 table |

Stanford scans carry no normals or UVs. The harness derives area-weighted vertex
normals and a box-projected UV set so that both builders see the same
position, normal and one UV set, the only layout Bevy accepts. glTF assets use
their own normals and first UV set when present.

Poly Haven textures are not downloaded; the downloaded glTF has its texture
references removed so that a reader never has to open an image.

The attribute/material rerun uses FlightHelmet's per-primitive material indices
as u16 triangle slots. Its texture files are unnecessary: normal and UV accessors
and the material index are read from the glTF and binary buffer only. The rerun
checks every decoded vertex against its exact builder source reference, including
seam duplicates, and every decoded material range against the source slots.

The foliage guard uses the same first mesh of Poly Haven `fern_02` as the
impostor measurements in `docs/USING.md`: 2,384 triangles, CC0 1.0, with the
separate 1k `Alpha` PNG converted losslessly to an 8-bit gray plane. Its glTF
material is `MASK`, cutoff 0.5 and double sided. Texture data stays external.
`l5_inputs.py` prepares the first primitive at extent 2 from the caller's glTF,
binary buffer and decoded alpha plane; `vg_foliage_measure` emits CPU cut and
coverage receipts. The source texture's row zero maps directly to UV v=0.
The fixture uses nearest repeat alpha sampling; it does not model mipmaps.
