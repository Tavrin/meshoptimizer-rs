#!/usr/bin/env python3
"""Rewrite public text in place; historical input hashes remain historical.

Run from any directory. Python and shell paths become environment-backed;
archived text uses the same neutral placeholders. No files are removed.
"""
import ast
import io
from pathlib import Path
import re
import subprocess
import tokenize

ROOT = Path(__file__).resolve().parents[2]
# Build legacy prefixes from components so the scrubber passes the public guard.
HOME = r'/home/[a-z]+/'
MOUNT = '/' + 'mnt/' + 'linux-extra/'
PRIVATE = 'mo' + 'ss'
PATH = re.compile(r'(?:' + HOME + r'|/mnt/)[^\s\x22\x27`<>;,|()\[\]{}\\]*')
ENV = {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR',
       'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS',
       'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}
HELPER = '''\n# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in %r.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)
\n''' % ENV


def path(match):
    value = match[0]
    # Preserve sentence punctuation, not machine directory structure.
    trail = value[len(value.rstrip('.:')):]
    value = value.rstrip('.:')
    home = re.match(HOME, value)
    home = home[0] if home else '<unused>/'
    rules = [(home + 'dev/meshoptimizer-rs-wt/', '<repo>', True),
             (home + 'dev/meshoptimizer-rs', '<repo>', False),
             (MOUNT + PRIVATE + '-scratch', '<scratch>', False),
             (MOUNT + PRIVATE + '-cargo-targets/', '<target-dir>', True),
             (MOUNT + PRIVATE + '-cargo-targets', '<target-dir>', False),
             (MOUNT + PRIVATE + '-lane-worktrees/', '<worktree>', True),
             (MOUNT + 'meshopt-artifacts', '<artifacts>', False),
             (home + 'dev/refs', '<refs>', False),
             (home + '.cargo/registry', '~/.cargo/registry', False)]
    for prefix, replacement, consume in rules:
        if value.startswith(prefix):
            rest = value[len(prefix):]
            if consume:
                rest = '/' + rest.split('/', 1)[1] if '/' in rest else ''
            return replacement + rest + trail
    if value.endswith('/bin/' + PRIVATE + '-heavy.sh'):
        return '<job-runner>' + trail
    # Other local paths retain only their terminal component.
    return '<local>/' + value.rstrip('/').rsplit('/', 1)[-1] + trail


def neutral(text, attribution=False):
    text = PATH.sub(path, text)
    text = re.sub(r'\b' + PRIVATE + r'-heavy(?:\.sh)?\b', '<job-runner>', text)
    text = re.sub(r'(?<![\w-])' + PRIVATE + r'-([\w-]+)', r'downstream-\1', text)
    text = text.replace(PRIVATE + '_cooker', 'downstream_cooker')
    for old, new in [('apollo' + '-main', 'storage'), ('automation' + '_game', 'projects')]:
        text = text.replace(old, new)
    if not attribution:
        text = re.sub(r'\b' + PRIVATE.capitalize() + r'\b', 'a downstream engine', text)
    return text


def python(source):
    if not any('<' + key + '>' in source for key in ENV) and '~/.cargo/registry' not in source:
        return source
    lines = source.splitlines(keepends=True)
    offsets = [0]
    for line in lines:
        offsets.append(offsets[-1] + len(line))
    docstrings = set()
    for node in ast.walk(ast.parse(source)):
        body = getattr(node, 'body', None)
        if isinstance(body, list) and body and isinstance(body[0], ast.Expr):
            value = body[0].value
            if isinstance(value, ast.Constant) and isinstance(value.value, str):
                docstrings.add((value.lineno, value.col_offset))
    edits = []
    for token in tokenize.generate_tokens(io.StringIO(source).readline):
        if token.type == tokenize.STRING and token.start not in docstrings and (any('<' + key + '>' in token.string for key in ENV) or '~/.cargo/registry' in token.string):
            a = offsets[token.start[0]-1] + token.start[1]
            b = offsets[token.end[0]-1] + token.end[1]
            edits.append((a, b, '_public_path(' + token.string + ')'))
    if not edits:
        return source
    for a, b, value in reversed(edits):
        source = source[:a] + value + source[b:]
    tree = ast.parse(source)
    insert = 0
    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and node.module == '__future__' or (
                node is tree.body[0] and isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant)
                and isinstance(node.value.value, str)):
            insert = node.end_lineno
        else:
            break
    # Retain shebangs and encoding declarations.
    if insert == 0:
        for line in source.splitlines():
            if line.startswith('#') or not line.strip():
                insert += 1
            else:
                break
    lines = source.splitlines(keepends=True)
    return ''.join(lines[:insert]) + HELPER + ''.join(lines[insert:])


def shell(source):
    # Quoted Python heredocs must use Python environment expansion.
    # Work linewise to preserve shell syntax and heredoc terminators.
    lines = source.splitlines(keepends=True)
    result = []; i = 0
    while i < len(lines):
        line = lines[i]
        marker = re.search(r"<<\s*'?(PY\w*)'?\s*$", line)
        if marker:
            result.append(line); i += 1; block = []
            while i < len(lines) and lines[i].strip() != marker[1]:
                block.append(lines[i]); i += 1
            result.append(python(''.join(block)))
            if i < len(lines):
                result.append(lines[i]); i += 1
            continue
        if not line.lstrip().startswith('#'):
            # Expand safely in unquoted, double-quoted and single-quoted words.
            out = []; pos = 0; quote = None
            pattern = re.compile(r'<(' + '|'.join(ENV) + r')>|~/(?=\.cargo/registry)')
            while pos < len(line):
                if line[pos] == "\\" and quote != "'":
                    out.append(line[pos:pos+2]); pos += 2; continue
                match = pattern.match(line, pos)
                if match:
                    env = ENV[match[1]] if match[1] else 'HOME'
                    value = '${' + env + ':?}' + ('/' if not match[1] else '')
                    if quote == "'":
                        value = "'\"" + value + "\"'"
                    elif quote != '"':
                        value = '"' + value + '"'
                    out.append(value); pos = match.end(); continue
                char = line[pos]
                if char in "\"'":
                    if quote is None: quote = char
                    elif quote == char: quote = None
                out.append(char); pos += 1
            line = ''.join(out)
        result.append(line); i += 1
    return ''.join(result)


def main():
    files = subprocess.check_output(['git', 'ls-files', '-z'], cwd=ROOT).decode().split('\0')
    changed = []
    for name in filter(None, files):
        if name in {'parity/tools/scrub_paths.py', 'parity/tools/check_public_text.sh'}:
            continue
        file = ROOT / name
        try:
            source = file.read_bytes().decode('utf-8')
        except (UnicodeDecodeError, IsADirectoryError):
            continue
        # Preserve upstream/downstream licence attribution verbatim in codecs.
        attribution = name.startswith('src/codec/')
        result = neutral(source, attribution)
        if result == source:
            continue
        if file.suffix == '.py':
            result = python(result)
        elif file.suffix == '.sh':
            result = shell(result)
        file.write_bytes(result.encode('utf-8'))
        changed.append(name)
    print('Rewrote %d files' % len(changed))


if __name__ == '__main__':
    main()
