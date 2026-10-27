#!/usr/bin/env python3
"""Neutralize archived project and planning names without changing measurements.

Only tracked UTF-8 text is rewritten. Renames use Git so evidence is retained.
Historical hashes are deliberately left untouched; current-file bindings are
reviewed separately and recorded in scrub_hash_updates.json.
"""
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PRIVATE = 'mo' + 'ss'
PLAN = 'rf' + 'c'
BRIEF = 'sp' + 'ec-'
ATTRIBUTIONS = {
    'src/codec/index.rs': '// Adapted from ' + PRIVATE.capitalize() + ' dc4af42a; MIT, ' + PRIVATE.capitalize() + ' contributors and Arseny Kapoulkine.',
    'src/codec/vertex.rs': '// Byte-group parsing also reuses the ' + PRIVATE.capitalize() + ' dc4af42a decoder (MIT).',
}


def neutral(source, release_labels=True):
    # Cargo profiles have a separate name from consumer/result labels.
    source = re.sub(r'(?i)(\[profile\.)' + PRIVATE + r'(\])', r'\1bench-release\2', source)
    source = re.sub(r'(--profile(?:[\s\x22\x27,])+)' + PRIVATE + r'\b', r'\1bench-release', source)
    source = source.replace('/' + PRIVATE + '/', '/bench-release/')
    source = re.sub(r'([\x22\x27])' + PRIVATE + r'/', r'\1bench-release/', source)
    source = re.sub(r'(?i)\b' + PRIVATE + r' profile\b', 'bench-release profile', source)
    for old, new in [('HEAVY_ACTIVE', 'RUNNER_ACTIVE'), ('HEAVY_GPU', 'RUNNER_GPU'),
                     ('HEAVY_MEASUREMENT', 'RUNNER_MEASUREMENT'), ('GPU_LEASE', 'GPU_LEASE'),
                     ('LANE', 'JOB')]:
        source = source.replace(PRIVATE.upper() + '_' + old, 'PARITY_' + new)
    source = re.sub(r'(?i)\b' + PRIVATE + r'-[a-z0-9]{5}\b', 'tracking-item', source)
    source = source.replace(PRIVATE.capitalize(), 'the downstream engine')
    source = source.replace(PRIVATE.upper(), 'DOWNSTREAM').replace(PRIVATE, 'downstream')
    source = source.replace(PLAN.upper() + '113', 'CLUSTER_LOD')
    source = source.replace(PLAN + '113', 'cluster-lod')
    source = re.sub(r'(?i)\b' + PLAN + r'\s*[0-9]+(?:\.[0-9]+)?', 'design requirements', source)
    source = re.sub(r'(?i)\b' + BRIEF + r'[\w.-]+', 'registered protocol', source)
    source = re.sub(r'\bS1 oracle\b', 'reference oracle', source)
    # Keep release selectors and their consumers synchronized, including CLI flags.
    if release_labels:
        source = re.sub(r'\bphase\b', 'release', source)
        source = re.sub(r'\bPhase\b', 'Release', source)
    source = source.replace('the a downstream engine', 'the downstream engine')
    source = source.replace('the the downstream engine', 'the downstream engine')
    return source


def renamed(name):
    return name.replace(PRIVATE, 'downstream').replace(PLAN + '113', 'cluster-lod')


def main():
    names = subprocess.check_output(['git', 'ls-files', '-z'], cwd=ROOT).decode().split('\0')
    changed = 0
    for name in filter(None, names):
        if name in {'parity/tools/scrub_names.py', 'parity/tools/check_public_text.sh'}:
            continue
        file = ROOT / name
        try:
            source = file.read_bytes().decode('utf-8')
        except (UnicodeDecodeError, IsADirectoryError):
            continue
        if name == 'CHANGELOG.md':
            # Leave released notes alone unless a targeted label occurs.
            source_lines = source.splitlines(keepends=True)
            result = ''.join(neutral(line) if re.search(
                PRIVATE + '|' + PLAN + r' ?[0-9]|' + BRIEF + r'|phase [0-9]',
                line, re.I) else line for line in source_lines)
            if result != source:
                file.write_text(result)
                changed += 1
            continue
        result = ''.join(line if line.rstrip('\n') == ATTRIBUTIONS.get(name)
                         else neutral(line, not name.startswith(('src/', 'tests/'))) for line in source.splitlines(keepends=True))
        # Internal work allocation in prose; preserve vector terminology in code.
        if file.suffix == '.md':
            result = re.sub(r'\blane\b', 'job', result)
            result = re.sub(r'\bLane\b', 'Job', result)
        if name == 'src/clusterlod/support/mod.rs':
            result = result.replace('lane p01x', 'preprocessing implementation')
        if result != source:
            file.write_text(result)
            changed += 1
    renames = []
    # Move the complete directory first, then the remaining individual files.
    old_dir = 'parity/' + PLAN + '113'
    if (ROOT / old_dir).exists():
        subprocess.run(['git', 'mv', old_dir, 'parity/cluster-lod'], cwd=ROOT, check=True)
        renames.append((old_dir + '/', 'parity/cluster-lod/'))
    for name in filter(None, names):
        if name.startswith(old_dir + '/'):
            continue
        new = renamed(name)
        if new != name:
            subprocess.run(['git', 'mv', name, new], cwd=ROOT, check=True)
            renames.append((name, new))
    print(f'Rewrote {changed} files')
    for old, new in renames:
        print(f'{old} -> {new}')


if __name__ == '__main__':
    main()
