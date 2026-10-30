"""Construct the second experiment from the frozen baseline after campaign 1.

Replaces repeated finalization bounds reads with the already computed ordered
bounds, keeping the exact work scan. P4_SCRATCH names the retained baseline.
Only the two simplifier sources are written; no Git state is changed.
"""
import os
from pathlib import Path
import json
import hashlib
import shutil
import subprocess

ROOT=Path(__file__).resolve().parents[2]
LANE=Path(__file__).resolve().parent
S=Path(os.environ['P4_SCRATCH'])
binding=json.loads((S/'experiment1/build-binding.json').read_text())['candidate-default']['sources']
for name in ['src/simplify.rs','src/simplify_extra.rs']:
    assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==binding[name]

def edit(s,old,new):
    assert s.count(old)==1,old
    return s.replace(old,new,1)

s=(S/'base/src/simplify.rs').read_text()
s=edit(s,'pub(crate) struct State {','''// Original-space bounds from the same ordered scan used for normalization.
// Update finalization reuses them before it mutates caller positions.
struct PositionBounds {
    lo: [f32; 3],
    hi: [f32; 3],
}
pub(crate) struct State {''')
s=edit(s,'let (result, state) = run_state(', 'let (result, state, _) = run_state(')
s=edit(s,') -> Result<(SimplifyResult, State), Error> {',') -> Result<(SimplifyResult, State, PositionBounds), Error> {')
s=edit(s,'''        s,
    ))''','''        s,
        PositionBounds { lo, hi },
    ))''')
s=edit(s, '    #[test]\n    fn preflight_preserves_the_first_reservations_limit_error()', '''    #[test]
    fn update_bounds_preserve_original_sparse_order_and_zero_signs() {
        let p = [
            [100.0; 3], // Unreferenced; dense bounds must still include it.
            [0.0, -0.0, 0.0],
            [1.0, 0.0, 0.0],
            [-0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let indices = [3, 2, 1, 1, 2, 4];
        for (options, expected_lo, expected_hi) in [
            (SimplifyOptions::EMPTY, [0.0f32, -0.0, 0.0], [100.0f32; 3]),
            (SimplifyOptions::SPARSE, [-0.0f32, 0.0, 0.0], [1.0f32; 3]),
        ] {
            let mut out = [0; 6];
            let mut ws = Workspace::default();
            let mut work = ws.begin();
            let (_, _, bounds) = run_state(
                &mut out,
                Input::borrowed(&indices),
                Positions::from_packed(&p),
                None,
                &[],
                None,
                SimplifySettings {
                    target_index_count: indices.len(),
                    target_error: 1.0,
                    options,
                },
                &mut ws,
                &mut work,
                0,
                true,
            ).unwrap();
            assert_eq!(bounds.lo.map(f32::to_bits), expected_lo.map(f32::to_bits));
            assert_eq!(bounds.hi.map(f32::to_bits), expected_hi.map(f32::to_bits));
        }
    }

    #[test]
    fn preflight_preserves_the_first_reservations_limit_error()''')
(ROOT/'src/simplify.rs').write_text(s)
s=(S/'base/src/simplify_extra.rs').read_text()
s=edit(s,') -> Result<(SimplifyResult, State), Error> {',') -> Result<(SimplifyResult, State, PositionBounds), Error> {')
s=edit(s,'let (r, mut state) = run_state(', 'let (r, mut state, bounds) = run_state(')
s=edit(s,'Ok((r, state))','Ok((r, state, bounds))')
s=edit(s,'let (r, s) = run_state(', 'let (r, s, bounds) = run_state(')
start=s.index('        let mut lo = [f32::MAX; 3];', s.index('pub fn simplify_with_update('))
end=s.index('        let mut extent = 0.0;',start)
s=s[:start]+'''        // The normalization scan visited the same original-space positions
        // in this order (including the sparse map). Caller positions are still
        // untouched, and all those reads are in bounds. Keep every work visit,
        // including the limited failure prefix, without repeating the reads.
        work.scan(s.p.iter(), |_| Ok(()))?;
        let PositionBounds { lo, hi } = bounds;
'''+s[end:]
(ROOT/'src/simplify_extra.rs').write_text(s)
subprocess.run(['rustfmt','--edition','2021',str(ROOT/'src/simplify.rs'),str(ROOT/'src/simplify_extra.rs')],check=True)
snapshot=S/'experiment2-source';snapshot.mkdir()
for name in ['src','docs']:shutil.copytree(ROOT/name,snapshot/name)
for name in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(ROOT/name,snapshot/name)
(LANE/'experiment2.patch').write_bytes(subprocess.check_output(['git','--no-optional-locks','diff','--','src/simplify.rs','src/simplify_extra.rs'],cwd=ROOT))
