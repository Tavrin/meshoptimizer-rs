"""Predeclared, distribution-free stopping rule and physical-core admission.

The confidence interval is an anytime interval for a population median. At
look n, the two-sided binomial error budget is alpha/(n(n+1)); summing over all
looks costs at most alpha. Each case receives 0.05 / number_of_cases. A codec
case splits that budget between the ratio and the registered floor when one
applies, and the ratio splits its budget between backend medians. No normality
or bounded-ratio assumption is made. Coverage assumes independent stationary
draws within each backend sequence (or the paired-ratio sequence). Correlation
and changing load can invalidate that sampling model. A finite maximum sample
count is still required for a point-estimate verdict.
"""
import math
import os
from pathlib import Path
import statistics
import time


def median_interval(values, alpha):
    """Two-sided simultaneous order-statistic interval at this look."""
    values = sorted(values)
    n = len(values)
    if n == 0 or not 0 < alpha < 1:
        raise ValueError('invalid median interval input')
    tail = alpha / (2 * n * (n + 1))
    cdf = 0.0
    lower_rank = -1
    for k in range(n // 2 + 1):
        cdf += math.comb(n, k) * 0.5 ** n
        if cdf <= tail:
            lower_rank = k
        else:
            break
    if lower_rank < 0:
        return 0.0, math.inf
    return values[lower_rank], values[n - lower_rank - 1]


def ratio_interval(rust, cpp, alpha, paired=True):
    if paired:
        ratios = [a / b for a, b in zip(rust, cpp, strict=True)]
        lo, hi = median_interval(ratios, alpha)
        return statistics.median(ratios), lo, hi
    rlo, rhi = median_interval(rust, alpha / 2)
    clo, chi = median_interval(cpp, alpha / 2)
    return (statistics.median(rust) / statistics.median(cpp),
            rlo / chi, rhi / clo if clo > 0 else math.inf)


def stop(rust, cpp, alpha, paired=True, minimum=12, maximum=80):
    n = len(rust)
    if n != len(cpp) or n == 0 or any(not math.isfinite(x) or x <= 0 for x in rust + cpp):
        raise ValueError('invalid paired timings')
    estimate, lo, hi = ratio_interval(rust, cpp, alpha, paired)
    gate = 'below' if hi < 1.5 else 'above' if lo > 1.5 else 'unresolved'
    if n < minimum:
        reason = 'minimum'
    elif hi < 1.5:
        reason = 'clearly_below_case_bar'
    elif lo > 1.5:
        reason = 'clearly_above_case_bar'
    elif n >= maximum:
        reason = 'maximum'
    else:
        reason = 'continue'
    return {'estimate': estimate, 'lower': lo, 'upper': hi,
            'case_bar_class': gate, 'reason': reason, 'pairs': n,
            'stopped': reason not in ('minimum', 'continue')}


def codec_stop(decision, rust, alpha, minimum, decoded_bytes, raw_scalar,
               minimum_pairs=12, maximum_pairs=80):
    """Resolve only the bars that apply to a registered 0.2 decoder case."""
    if minimum is None:
        floor_class = 'unregistered'
    else:
        lo, hi = median_interval(rust, alpha)
        ceiling = decoded_bytes / minimum
        decision['registered_floor_seconds_ceiling'] = ceiling
        decision['registered_floor_interval_seconds'] = [lo, hi]
        floor_class = ('above_floor' if hi < ceiling else
                       'below_floor' if lo > ceiling else 'unresolved')
    decision['registered_floor_class'] = floor_class
    resolved = (floor_class != 'unresolved' and
                (not raw_scalar or decision['case_bar_class'] != 'unresolved'))
    decision['stopped'] = (len(rust) >= minimum_pairs and resolved) or len(rust) >= maximum_pairs
    if len(rust) < minimum_pairs:
        decision['reason'] = 'minimum'
    elif not resolved and len(rust) >= maximum_pairs:
        decision['reason'] = 'maximum'
    elif resolved and not raw_scalar:
        decision['reason'] = 'registered_floor_resolved'
    elif not decision['stopped']:
        decision['reason'] = 'continue_registered_floor_or_raw_case_bar'
    return decision


def _ticks():
    result = {}
    for line in Path('/proc/stat').read_text().splitlines():
        fields = line.split()
        if fields and fields[0].startswith('cpu') and fields[0][3:].isdigit():
            numbers = [int(x) for x in fields[1:9]]
            result[int(fields[0][3:])] = (sum(numbers), numbers[3] + numbers[4])
    return result


def _frequency(cpu):
    path = Path(f'/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_cur_freq')
    return int(path.read_text()) if path.exists() else None


def physical_cores(seconds=1.0, requested=None):
    """One CPU per least-busy physical core; reserve at least four whole cores."""
    allowed = os.sched_getaffinity(0)
    before = _ticks()
    time.sleep(seconds)
    after = _ticks()
    groups = {}
    all_siblings = {}
    for cpu in allowed:
        topo = Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
        key = (int((topo / 'physical_package_id').read_text()),
               int((topo / 'core_id').read_text()))
        total = after[cpu][0] - before[cpu][0]
        idle = after[cpu][1] - before[cpu][1]
        if total <= 0 or not 0 <= idle <= total:
            raise ValueError('invalid CPU telemetry interval')
        groups.setdefault(key, []).append({'cpu': cpu, 'busy_fraction': 1-idle/total,
                                             'frequency_khz': _frequency(cpu)})
        members = []
        for part in (topo / 'thread_siblings_list').read_text().strip().split(','):
            lo, separator, hi = part.partition('-')
            members.extend(range(int(lo), int(hi) + 1) if separator else [int(lo)])
        all_siblings[key] = sorted(set(members))
    cores = []
    for (package, core), siblings in groups.items():
        loads = []
        for cpu in all_siblings[(package, core)]:
            if cpu not in before or cpu not in after:
                raise ValueError('SMT sibling telemetry unavailable')
            total = after[cpu][0] - before[cpu][0]
            idle = after[cpu][1] - before[cpu][1]
            if total <= 0 or not 0 <= idle <= total:
                raise ValueError('invalid SMT sibling telemetry interval')
            loads.append(1 - idle / total)
        busy = max(loads)
        selected = min(siblings, key=lambda s: (s['busy_fraction'], s['cpu']))
        cores.append({'package': package, 'core': core, 'cpu': selected['cpu'],
                      'siblings': all_siblings[(package, core)],
                      'busy_fraction': busy, 'frequency_khz': selected['frequency_khz']})
    cores.sort(key=lambda c: (c['busy_fraction'], c['package'], c['core']))
    default = max(1, len(cores) - 2)
    cap = max(0, len(cores) - 4)
    count = min(default if requested is None else requested, cap)
    if count < 1:
        raise ValueError('fewer than five allowed physical cores; cannot leave four free')
    # A core with a busy sibling is unavailable even if the selected sibling is idle.
    eligible = [c for c in cores if c['busy_fraction'] < 0.5]
    chosen = eligible[:count]
    if not chosen:
        raise ValueError('no physical core below 50% busy at admission')
    return {'selected': chosen, 'all': cores, 'default_workers': default,
            'reserved_physical_cores': len(cores)-len(chosen),
            'selection_seconds': seconds, 'load_average': os.getloadavg()}


def core_load(cpu):
    ticks = _ticks()[cpu]
    return {'unix': time.time(), 'total_ticks': ticks[0], 'idle_ticks': ticks[1],
            'frequency_khz': _frequency(cpu), 'load_average': os.getloadavg()}


def add_core_delta(before, after):
    total = after['total_ticks']-before['total_ticks']
    idle = after['idle_ticks']-before['idle_ticks']
    return {**after, 'busy_fraction_since_before': 1-idle/total if total > 0 else None}
