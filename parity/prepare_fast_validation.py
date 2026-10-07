#!/usr/bin/env python3
"""Create a source-only validation export without changing any Git state."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import shutil
import subprocess
import tarfile
import zipfile

from fast_qualify import ART, ROOT
from fast_validate import read_record, source_hashes


def checked_path(root, relative):
    destination = (root / relative).resolve()
    if root.resolve() not in destination.parents:
        raise ValueError('archive path escapes export: ' + relative)
    return destination


def write_member(root, name, data, mode=0o644):
    destination = checked_path(root, name)
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(data)
    destination.chmod(mode & 0o777)


def export_git(revision, root):
    data = subprocess.check_output(['git', '-C', str(ROOT), 'archive', '--format=tar', revision])
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        for member in archive:
            if member.isfile():
                write_member(root, member.name, archive.extractfile(member).read(), member.mode)


def export_p02(path, root):
    with zipfile.ZipFile(path) as archive:
        for member in archive.infolist():
            if member.filename.startswith('repo/') and not member.is_dir():
                mode = (member.external_attr >> 16) & 0o777 or 0o644
                write_member(root, member.filename[5:], archive.read(member), mode)


def export_source_tar(path, root):
    with tarfile.open(path, 'r:*') as archive:
        for member in archive:
            if member.isfile():
                write_member(root, member.name, archive.extractfile(member).read(), member.mode)


def verify_library(root, full_location):
    full, _ = read_record(full_location)
    source = full.get('identity', full.get('identities', full))
    expected = source_hashes(source)
    if not expected:
        raise ValueError('full record has no library source hashes')
    mismatched = [relative for relative, digest in expected.items()
                  if not (root / relative).is_file() or
                  hashlib.sha256((root / relative).read_bytes()).hexdigest() != digest]
    if mismatched:
        raise ValueError('export is not the full record revision: ' + ', '.join(mismatched))
    return len(expected)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('label', help='plain export directory name')
    choice = parser.add_mutually_exclusive_group(required=True)
    choice.add_argument('--git-revision')
    choice.add_argument('--p02-source-zip', type=Path)
    choice.add_argument('--source-tar', type=Path, help='retained sources.tar.gz; read only')
    parser.add_argument('--full-record', required=True)
    args = parser.parse_args()
    if not args.label.replace('-', '').replace('_', '').isalnum():
        parser.error('label must be plain alphanumeric, dash or underscore')
    root = ART / 'exports' / args.label
    root.mkdir(parents=True, exist_ok=False)
    if args.git_revision:
        export_git(args.git_revision, root)
    elif args.p02_source_zip:
        export_p02(args.p02_source_zip, root)
    else:
        export_source_tar(args.source_tar, root)
    count = verify_library(root, args.full_record)
    for filename in ('fast_qualify.py', 'fast_stats.py', 'fast_validate.py', 'test_fast_stats.py'):
        shutil.copyfile(ROOT / 'parity' / filename, root / 'parity' / filename)
    if args.p02_source_zip:
        # Only the case-generation refactor changes; library files stay exact.
        shutil.copyfile(ROOT / 'parity/codec/measure.py', root / 'parity/codec/measure.py')
    print(json.dumps({'export': str(root), 'source_files_matched': count,
                      'full_record': args.full_record}, indent=2))


if __name__ == '__main__':
    main()
