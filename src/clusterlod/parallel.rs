// Speculate only on immutable per-level inputs. Nothing observable is committed
// until every group has succeeded and ordered assembly reaches that ordinal.
use super::*;
use crate::processing::OwnedBytes;
use crate::{Limits, Usage};
use rayon::prelude::*;

pub(super) fn eligible(c: Config, limits: Limits) -> bool {
    !c.simplify_dilate_borders && limits.max_bytes == usize::MAX && limits.max_work == u64::MAX
}

pub(super) struct Clustered {
    pub output: (Vec<Pending>, Vec<u32>),
    pub usage: Usage,
    pub live: usize,
}
pub(super) struct Computed {
    pub prepared: Prepared,
    pub usage: Usage,
    pub live: usize,
    pub clustered: Option<Clustered>,
}

fn compute(
    c: Config,
    mesh: &Mesh<'_>,
    locks: &[support::VertexFlags],
    group: &[Pending],
    idx: &[u32],
    moderate: bool,
) -> Result<Computed, Error> {
    let limits = Limits {
        max_bytes: usize::MAX,
        max_work: u64::MAX,
    };
    let mut workspace = Workspace::new(limits);
    let prepared = prepare_group(
        group,
        idx,
        mesh,
        locks,
        c,
        &mut Context::new(&mut workspace)?,
    )?;
    let usage = workspace.usage();
    let live = prepared
        .merged_idx
        .owned_bytes()?
        .checked_add(prepared.gb.owned_bytes()?)
        .and_then(|n| n.checked_add(prepared.simplified.capacity().checked_mul(4)?))
        .ok_or(Error::SizeOverflow)?;
    let clustered = if prepared.stuck {
        None
    } else {
        // Use a fresh context: ordered assembly adds the still-live preparation
        // and callback output to this stage's relative peak. The callback's
        // unknown ID is applied later; it is never read by clusterize.
        let output = clusterize(
            &prepared.simplified,
            Positions::from_packed(mesh.positions),
            c,
            -1,
            moderate,
            &mut Context::new(&mut workspace)?,
        )?;
        let usage = workspace.usage();
        let live = output
            .0
            .owned_bytes()?
            .checked_add(output.1.owned_bytes()?)
            .ok_or(Error::SizeOverflow)?;
        Some(Clustered {
            output,
            usage,
            live,
        })
    };
    Ok(Computed {
        prepared,
        usage,
        live,
        clustered,
    })
}

pub(super) fn level(
    c: Config,
    mesh: &Mesh<'_>,
    locks: &[support::VertexFlags],
    clusters: &[Pending],
    idx: &[u32],
    offsets: &[u32],
    moderate: bool,
    remaining_work: u64,
) -> Option<Vec<Computed>> {
    if offsets.len() <= 2 {
        return None;
    }
    // Fallible slots; allocation failure simply chooses the original serial
    // loop before any callback/accounting from this level is visible.
    let mut slots = Vec::new();
    slots.try_reserve_exact(offsets.len() - 1).ok()?;
    slots.resize_with(offsets.len() - 1, || Err(Error::AllocationFailed));
    slots
        .par_iter_mut()
        .enumerate()
        .for_each(|(ordinal, slot)| {
            #[cfg(feature = "parity-internals")]
            if std::env::var("MESHOPT_CLOD_SCHEDULE").as_deref() == Ok("fail") && ordinal == 0 {
                *slot = Err(Error::NumericalFailure);
                return;
            }
            #[cfg(feature = "parity-internals")]
            schedule(ordinal, offsets.len() - 1);
            *slot = compute(
                c,
                mesh,
                locks,
                &clusters[offsets[ordinal] as usize..offsets[ordinal + 1] as usize],
                idx,
                moderate,
            );
        });
    // Reuse the slots allocation is not possible across these different element
    // types. Reserve fallibly again, still before touching the serial context.
    let mut result = Vec::new();
    result.try_reserve_exact(slots.len()).ok()?;
    for slot in slots {
        result.push(slot.ok()?);
    }
    if !fits_serial_work(&result, clusters, mesh.positions.len(), c, remaining_work) {
        return None;
    }
    Some(result)
}

fn fits_serial_work(
    results: &[Computed],
    clusters: &[Pending],
    vertices: usize,
    c: Config,
    remaining: u64,
) -> bool {
    let bound = (|| {
        // At most one visit per allocating callback. Caller callbacks cannot
        // charge the private context. Public bounds: position validation plus
        // topology, corner, sphere and normal visits <= vertices + 6*indices.
        let mut work = u64::try_from(results.len()).ok()?;
        for result in results {
            work = work.checked_add(result.usage.work)?;
            if let Some(clustered) = &result.clustered {
                work = work.checked_add(clustered.usage.work)?;
            }
        }
        if c.optimize_bounds {
            for cl in clusters.iter().filter(|cl| cl.refined != -1) {
                let bounds = u64::try_from(vertices)
                    .ok()?
                    .checked_add(u64::try_from(cl.count).ok()?.checked_mul(6)?)?;
                work = work.checked_add(bounds)?;
            }
        }
        Some(work)
    })();
    bound.is_some_and(|work| work <= remaining)
}

// Test-driver-only perturbation; production builds never read environment state.
#[cfg(feature = "parity-internals")]
fn schedule(ordinal: usize, count: usize) {
    let delay = match std::env::var("MESHOPT_CLOD_SCHEDULE").as_deref() {
        Ok("reverse") => count - ordinal,
        Ok("zigzag") => {
            if ordinal.is_multiple_of(2) {
                count - ordinal
            } else {
                ordinal
            }
        }
        _ => return,
    };
    std::thread::sleep(std::time::Duration::from_micros(
        (delay.min(64) * 25) as u64,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn work_bound_falls_back_before_serial_exhaustion_or_overflow() {
        let result = Computed {
            prepared: Prepared {
                merged_idx: Vec::new(),
                gb: Vec::new(),
                simplified: Vec::new(),
                bounds: LodBounds::default(),
                stuck: true,
            },
            usage: Usage { bytes: 0, work: 10 },
            live: 0,
            clustered: None,
        };
        let mut c = default_config(4).unwrap();
        let results = [result];
        assert!(!fits_serial_work(&results, &[], 0, c, 10));
        assert!(fits_serial_work(&results, &[], 0, c, 11));
        c.optimize_bounds = true;
        let clusters = [Pending {
            refined: 0,
            count: 3,
            ..Pending::default()
        }];
        assert!(!fits_serial_work(&results, &clusters, 9, c, 37));
        assert!(fits_serial_work(&results, &clusters, 9, c, 38));
        let results = [Computed {
            usage: Usage {
                bytes: 0,
                work: u64::MAX,
            },
            ..results.into_iter().next().unwrap()
        }];
        assert!(!fits_serial_work(&results, &[], 0, c, u64::MAX));
    }
}
