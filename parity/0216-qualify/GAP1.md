# Gap 1 enumeration

The driver compares the frozen candidate and baseline in all-features and
no-default modes. Each comparison checks the `Result`, complete output and
mutable input buffers, and both work and byte usage. For every input and API it
performs an unlimited run followed by every integer work limit through the
larger unrestricted cold/reused usage plus one. Each limit observes two calls
on the same workspace: initially empty, then reused after the first call. A
failed first call may leave a partially populated or empty workspace; it is not
represented as a successful warmup.

The grid uses the six existing mesh generators, each at 16 and 65 vertices;
eight position layouts, three attribute layouts, widths 0, 1, 2, 3, 4, 8,
16 and 32; no flags, an all-zero flag slice, all flags, alternating flags,
first-only flags and last-only flags. The nonzero flag value is 7. It crosses
512 all-feature masks or 128 no-default masks. It checks the allocating and
caller-buffer variants of `simplify` and `simplify_with_attributes`, including
short caller buffers, and the in-place `simplify_with_update` API. The grid
contains 7,077,888 and 1,769,472 cells respectively. A cell is complete only
when every API and work limit has been compared. A coprime affine permutation
visits the grid exactly once while spreading any time-limited prefix across
its dimensions.

The emitted observation count counts cold and reused calls separately, including
the unlimited calls and any completed comparisons within the current cell;
completed-cell counts exclude that unfinished cell. Exit 124 records a time
limit, not a pass. A mismatch exits nonzero and is not silently skipped. Raw
logs, binaries, build logs and hashes are retained outside the repository.
