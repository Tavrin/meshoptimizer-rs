# Partition instruction qualification

Base is commit `bd0c891`, whose library source is the specified 0.2.13 arm; M7 is scalar upstream 1.3. The candidate must preserve every count, assignment, error and resource observation, including the full caller output on failure. The public API, allocation order and sizes, and safe Rust boundary are fixed.

Qualification uses the frozen partition corpus plus the partition calls in the real cluster-LOD chains. It records three `instructions:u` samples per arm and case. PASS requires candidate/M7 corpus geometric mean at most 1.15; otherwise PARTIAL requires candidate/base at most 0.90; otherwise NO-GAIN and revert. Any candidate/base case above 1.01 fails. At most two full campaigns; source edits after a campaign are limited to reverts. Sampling and counters use CPU-only shared admission with a 4 GB declaration. Completed records are retained and skipped on resumption.

Attribution precedes production edits and compares Rust with scalar M7 via instruction sampling and annotation. The exact differential compares two independently compiled Rust crates, both wrappers and limit failures; the pinned scalar C++ comparison checks count and assignments separately. The workload inventory must include the real chains before a qualification campaign begins.
