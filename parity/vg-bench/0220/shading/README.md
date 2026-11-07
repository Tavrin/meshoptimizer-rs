# Applied shading candidate

Continuation 3 applies [the original draft](DRAFT.patch) to the calibration
commit. The candidate adds sampled normal-loss displacement to MDAG 2 group
errors and binds its weight in builder revision 8. Container layouts are unchanged.
Static review also restores the public quality API's `NumericalFailure` for
undefined interpolated normals, which the draft accidentally changed.

**BLOCKED: no admission within 45 minutes; compilation, tests and comparison did not run.**
Weight zero remains a disabled placeholder, not a measured default selection.
No new normal/UV/tangent recommendation or shading-quality result is claimed.
Formatting and the public-text scan pass. Full gate and measurement status is
recorded in [the disposition](../REPORT.md) for this continuation.
