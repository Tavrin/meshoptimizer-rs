#!/usr/bin/env python3
"""Compare main and current bounded Workspace success for every boundary byte."""
import json
import differential as diff


def build(kind, root):
    project = diff.ART / f'budget-{kind}'
    project.mkdir(parents=True, exist_ok=True)
    source = diff.ROOT / 'parity/rfc113/budget.rs'
    (project / 'Cargo.toml').write_text(
        f'[package]\nname="rfc113-budget-{kind}"\nversion="0.0.0"\nedition="2021"\n'
        f'[[bin]]\nname="rfc113-budget-{kind}"\npath="{source}"\n'
        f'[dependencies]\nmeshoptimizer-rs={{path="{root}",default-features=false,features=["clusterlod"]}}\n')
    diff.command(['cargo', 'build', '--offline', '--release', '--manifest-path', project / 'Cargo.toml'])
    return diff.TARGET / 'release' / f'rfc113-budget-{kind}'


def query(binary, limits):
    data = ''.join(f'{limit}\n' for limit in limits).encode()
    output = diff.command([binary], input=data, capture_output=True).stdout.decode().splitlines()
    if len(output) != len(limits):
        raise RuntimeError('budget driver response count')
    return [(line.split()[0], int(line.split()[1])) for line in output]


def main():
    old_root = diff.ART / 'pre-optimization-main'
    if not (old_root / '.source-commit').is_file():
        raise SystemExit('run differential.py first to archive main')
    binaries = {'old': build('old', old_root), 'rust': build('current', diff.ROOT)}
    unlimited = {name: query(path, [2**63 - 1])[0] for name, path in binaries.items()}
    old_peak = unlimited['old'][1]
    limits = sorted(set([0, old_peak // 2, old_peak - 1024, old_peak + 1024]
                        + list(range(old_peak - 256, old_peak + 257))))
    outcomes = {name: query(path, limits) for name, path in binaries.items()}
    mismatches = [{'limit': limit, 'old': outcomes['old'][i][0], 'rust': outcomes['rust'][i][0]}
                  for i, limit in enumerate(limits)
                  if outcomes['old'][i][0] == 'Ok' and outcomes['rust'][i][0] != 'Ok']
    failure_kind_mismatches = [limit for i, limit in enumerate(limits)
                               if outcomes['old'][i][0] != 'Ok'
                               and outcomes['rust'][i][0] != 'Ok'
                               and outcomes['old'][i][0] != outcomes['rust'][i][0]]
    improvements = sum(outcomes['old'][i][0] != 'Ok' and outcomes['rust'][i][0] == 'Ok'
                       for i in range(len(limits)))
    result = {'schema': 'rfc113-budget/1', 'main_commit': (old_root / '.source-commit').read_text().strip(),
              'current_commit': diff.command(['git', 'rev-parse', 'HEAD'], cwd=diff.ROOT,
                                              capture_output=True, text=True).stdout.strip(),
              'source_sha256': diff.run.sha(diff.ROOT / 'parity/rfc113/budget.rs'),
              'executables': {name: diff.run.sha(path) for name, path in binaries.items()},
              'unlimited': unlimited, 'limits_checked': len(limits),
              'old_fail_new_success': improvements,
              'failure_kind_mismatches': failure_kind_mismatches[:8],
              'first_mismatches': mismatches[:8]}
    (diff.ART / 'budget.json').write_text(json.dumps(result, indent=2) + '\n')
    print('old peak', old_peak, 'new peak', unlimited['rust'][1],
          'limits', len(limits), 'regressions', len(mismatches),
          'error-kind mismatches', len(failure_kind_mismatches))
    if mismatches or failure_kind_mismatches or unlimited['rust'][1] > old_peak:
        raise SystemExit('bounded-memory regression')


if __name__ == '__main__':
    main()
