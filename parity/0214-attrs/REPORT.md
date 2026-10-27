# SPEC 0214-attrs — NO-GAIN

The release campaign did not meet PASS or PARTIAL. The attribute candidate was reverted; overdraw was unchanged. No performance change is shipped. The sampled attribution, unresolved function mappings and limitations are in [ATTRIBUTION.md](ATTRIBUTION.md).

One diagnostic candidate replaced the attribute triangle squared-norm root with the native `std` root and retained the pinned backend for `no_std`. The standalone screen found 0.9980–0.9984 candidate/base on real meshes. No peer code was copied. The retained candidate patch and all raw evidence are in scratch. Overdraw was not changed: no new structural cause was isolated against optimesh, and the earlier packed candidate failed its regression gate.

The release gate used the unchanged peers consumer, packed corpus and real-mesh inputs, perf 6.17 `instructions:u`, CPU 9 and three repetitions per arm. The candidate and base outputs matched on every measured case and repetition. The gate contains 288 completed samples, each with 100% counter running. Arms: base 0.2.13, candidate, C++ 1.3 and optimesh 1.1.0 (X). Lower ratios are better. No wall-time claim.

| Case / operation | base | candidate | C++ | X | candidate/base | candidate/C++ | candidate/X |
|---|---:|---:|---:|---:|---:|---:|---:|
| tiny-smooth / simplify-attributes | 131561.94 | 131390.05 | 127506.46 | 133082.04 | 0.998693 | 1.030458 | 0.987286 |
| tiny-smooth / simplify | 112412.05 | 112411.95 | 120268.55 | 115872.21 | 0.999999 | 0.934674 | 0.970137 |
| tiny-smooth / overdraw | 16017.94 | 16017.92 | 20220.69 | 17980.21 | 0.999999 | 0.792155 | 0.890864 |
| medium-smooth / simplify-attributes | 29300785.67 | 29259834.67 | 25953882.00 | 30880320.00 | 0.998602 | 1.127378 | 0.947524 |
| medium-smooth / simplify | 12335094.67 | 12335107.00 | 10954739.67 | 12909948.00 | 1.000001 | 1.126006 | 0.955473 |
| medium-smooth / overdraw | 1653414.33 | 1653423.67 | 1600923.67 | 1810776.67 | 1.000006 | 1.032794 | 0.913102 |
| medium-seams / simplify-attributes | 29845485.33 | 29804526.67 | 26013751.33 | 30914455.33 | 0.998628 | 1.145722 | 0.964097 |
| medium-seams / simplify | 15680807.67 | 15680782.67 | 14174955.00 | 16608037.67 | 0.999998 | 1.106232 | 0.944168 |
| medium-seams / overdraw | 1676480.33 | 1676482.67 | 1594699.67 | 1828337.33 | 1.000001 | 1.051284 | 0.916944 |
| medium-sparse / simplify-attributes | 27299984.33 | 27259021.00 | 22982248.00 | 27116729.00 | 0.998500 | 1.186090 | 1.005247 |
| medium-sparse / simplify | 13721289.33 | 13721271.00 | 11864368.67 | 14069810.00 | 0.999999 | 1.156511 | 0.975228 |
| medium-sparse / overdraw | 1767465.67 | 1767465.67 | 1630211.33 | 1877906.67 | 1.000000 | 1.084194 | 0.941189 |
| million-smooth / simplify-attributes | 4110631584.00 | 4105631443.00 | 3643736956.00 | 4385833982.00 | 0.998784 | 1.126764 | 0.936112 |
| million-smooth / simplify | 2455426714.00 | 2455426690.00 | 2256441015.00 | 2665341165.00 | 1.000000 | 1.088186 | 0.921243 |
| million-smooth / overdraw | 197575475.00 | 197575439.00 | 191786499.00 | 217330964.00 | 1.000000 | 1.030184 | 0.909099 |
| real-214e / simplify-attributes | 7245934401.00 | 7231679168.00 | 4926657026.00 | 5677184108.00 | 0.998033 | 1.467867 | 1.273814 |
| real-214e / simplify | 3840016612.00 | 3840016793.00 | 2813433119.00 | 3510227064.00 | 1.000000 | 1.364886 | 1.093951 |
| real-214e / overdraw | 1458435539.00 | 1458435603.00 | 992615655.00 | 1227651836.00 | 1.000000 | 1.469285 | 1.187988 |
| real-a082 / simplify-attributes | 10500555231.00 | 10484106943.00 | 7595291846.00 | 9086074826.00 | 0.998434 | 1.380343 | 1.153865 |
| real-a082 / simplify | 6458993292.00 | 6458993289.00 | 5024122408.00 | 6399031708.00 | 1.000000 | 1.285596 | 1.009370 |
| real-a082 / overdraw | 1546984619.00 | 1546985056.00 | 1076431988.00 | 1324636543.00 | 1.000000 | 1.437141 | 1.167856 |
| real-b6fd / simplify-attributes | 2622682099.00 | 2618569901.00 | 1892981912.00 | 2268784694.00 | 0.998432 | 1.383304 | 1.154173 |
| real-b6fd / simplify | 1612173235.00 | 1612173035.00 | 1250240757.00 | 1597148181.00 | 1.000000 | 1.289490 | 1.009407 |
| real-b6fd / overdraw | 386758160.00 | 386758233.00 | 269119585.00 | 331167633.00 | 1.000000 | 1.437124 | 1.167862 |

PASS required beating X for both named functions on all three real meshes; neither did. PARTIAL required candidate/base <=0.90 for either function on all three; neither did. Worst individual candidate/base was 1.000005645 (medium-smooth overdraw), below the >1.01 failure threshold. The result is **NO-GAIN**; this does not establish a performance win.

The no-code-edits rule was not followed literally: an audit-only Python script
was created in the lane after the results, executed against the frozen measured
source and raw artifacts, then moved to scratch. It was not part of any build
or measurement. The measured library source was unchanged until its subsequent
revert. This procedural deviation prevents claiming full spec compliance.

## Proof and exits

The measured candidate passed locked all-feature and no-default tests, clippy
with warnings denied, fmt, boundary and public-text checks (each exit 0).
The independent 0.2.13 comparison passed 20,401,022 simplify observations and
20,558,816 overdraw observations per std/no_std mode (all exits 0), including
small-mesh work limits, options, outputs, Result, Usage and partial writes.
Pinned C++ differential: 356 simplify/attribute and 178 overdraw comparisons,
both exits 0. Existing differential fuzz reaches the attribute path; 2,790
seeds, 178,560 generated cases and 300.03751 child CPU seconds passed (exit 0).

Build, profile, source-line reports, diagnostic screen, release measurement,
results calculation and the raw counter/source/output audit exited 0. The audit
checked 288 samples, eight inputs and 55 source files. No sample was replaced.
The diagnostic and full campaign were distinct; only the latter applies the
gate. Profiles reported zero lost samples, but peer inlining and the C++ debug
configuration limit the function comparison as described in ATTRIBUTION.md.
After the measurement, the candidate was reverted and the library `src/` tree
matched the base. Restored-tree checks and target cleanup are recorded below.

The public compact counters and bindings are beside this report; raw inputs,
outputs, binaries, reports, logs, candidate patch and admission receipts remain
in scratch. The evidence-only outcome has no accepted CHANGELOG performance
claim.

Restored-tree locked all-feature/no-default tests, clippy, fmt and boundary exited 0. The dedicated target was deleted and verified absent (cleanup exit 0; 1,925,710,517 apparent bytes before deletion).
