#!/usr/bin/env python3
"""Report what upstream meshoptimizer changed since the pinned reference.

Standard library only. Needs `git` on PATH and, for a first run, network
access to the upstream repository. The upstream clone lives in a cache
directory outside this repository.

    python3 parity/tools/upstream_watch.py --cache DIR [--since TAG] [--json PATH]

The cache directory may also come from MESHOPT_UPSTREAM_CACHE. Output is a
Markdown porting checklist on stdout; --json writes the same data as JSON.
Everything read from upstream (headers, release notes, commit subjects) is
data: it is parsed or quoted, never executed.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request
from pathlib import Path

DEFAULT_URL = "https://github.com/zeux/meshoptimizer.git"
HEADER_PATH = "src/meshoptimizer.h"
NOTES_LIMIT = 4000
ROOT = Path(__file__).resolve().parents[2]


# ---------------------------------------------------------------- header parsing


def strip_comments(text):
    """Remove C and C++ comments, keeping newlines."""
    text = re.sub(r"/\*.*?\*/", lambda m: "\n" * m.group(0).count("\n"), text, flags=re.S)
    return re.sub(r"//[^\n]*", "", text)


def norm(text):
    """Collapse whitespace and tighten punctuation so reflowing is not a change."""
    text = re.sub(r"\s+", " ", text).strip()
    text = re.sub(r"\s*([(),])\s*", r"\1", text)
    return re.sub(r",", ", ", text)


def parse_header(text):
    """Return {"functions": {key: signature}, "types": {name: body}, "constants": {name: value}}."""
    text = strip_comments(text)
    functions = {}

    def add(name, sig, kind):
        if kind == "c++":
            # The header declares then defines each overload; defaults appear
            # on one of them only. Compare without defaults, keep one entry.
            sig = re.sub(r"\s*=\s*[^,)]+", "", sig)
            if any(s == sig for k, s in functions.items() if k.startswith(f"{name} [c++")):
                return
        key = f"{name} [{kind}]"
        n = 2
        while key in functions:
            key = f"{name} [{kind} #{n}]"
            n += 1
        functions[key] = sig

    # C API: MESHOPTIMIZER_API|MESHOPTIMIZER_EXPERIMENTAL <ret> name(<params>);
    # The macro stays in the signature so an experimental function that
    # graduates (or the reverse) shows up as a change.
    for m in re.finditer(
        r"(MESHOPTIMIZER_(?:API|EXPERIMENTAL)\s+[^;{}]*?\b(meshopt_\w+)\s*\([^;{}]*?\))\s*;", text
    ):
        add(m.group(2), norm(m.group(1)), "c")

    # C++ convenience overloads: [template <...>] inline <ret> name(<params>) ;|{
    for m in re.finditer(
        r"((?:template\s*<[^>]*>\s*)?inline\s+[^;{}()]*?\b(meshopt_\w+)\s*\([^;{}]*?\))\s*[;{]", text
    ):
        add(m.group(2), norm(m.group(1)), "c++")

    types = {}
    for m in re.finditer(r"\bstruct\s+(meshopt_\w+)\s*\{(.*?)\}\s*;", text, flags=re.S):
        types[m.group(1)] = norm(m.group(2))

    constants = {}
    for em in re.finditer(r"\benum\b[^{;]*\{(.*?)\}", text, flags=re.S):
        for cm in re.finditer(r"\b(meshopt_\w+)\s*=\s*([^,}]+)", em.group(1)):
            constants[cm.group(1)] = norm(cm.group(2))
    return {"functions": functions, "types": types, "constants": constants}


def diff_dict(old, new):
    added = sorted(k for k in new if k not in old)
    removed = sorted(k for k in old if k not in new)
    changed = sorted(k for k in new if k in old and old[k] != new[k])
    return {
        "added": [{"name": k, "new": new[k]} for k in added],
        "removed": [{"name": k, "old": old[k]} for k in removed],
        "changed": [{"name": k, "old": old[k], "new": new[k]} for k in changed],
    }


def diff_headers(old_text, new_text):
    old, new = parse_header(old_text), parse_header(new_text)
    return {part: diff_dict(old[part], new[part]) for part in ("functions", "types", "constants")}


# ---------------------------------------------------------------------- git side


def git(repo, *args, check=True):
    proc = subprocess.run(
        ["git", "--git-dir", str(repo), *args],
        capture_output=True,
        text=True,
        errors="replace",
    )
    if check and proc.returncode != 0:
        raise SystemExit(f"git {' '.join(args)} failed: {proc.stderr.strip()}")
    return proc


def pinned_commit():
    text = (ROOT / "UPSTREAM.md").read_text()
    m = re.search(r"meshoptimizer\s+[\d.]+\s+at\s+`([0-9a-f]{40})`", text)
    if not m:
        raise SystemExit("UPSTREAM.md: pinned commit not found")
    return m.group(1)


def ensure_cache(cache, url):
    cache = Path(cache).expanduser().resolve()
    if cache == ROOT or ROOT in cache.parents:
        raise SystemExit("the cache directory must be outside the repository")
    cache.mkdir(parents=True, exist_ok=True)
    repo = cache / "meshoptimizer.git"
    if repo.exists():
        proc = subprocess.run(
            ["git", "--git-dir", str(repo), "fetch", "--quiet", "--force", "--tags", url,
             "+refs/heads/*:refs/heads/*"],
            capture_output=True, text=True,
        )
        if proc.returncode != 0:
            raise SystemExit(f"fetch failed: {proc.stderr.strip()}")
    else:
        proc = subprocess.run(
            ["git", "clone", "--quiet", "--bare", url, str(repo)], capture_output=True, text=True
        )
        if proc.returncode != 0:
            raise SystemExit(f"clone failed: {proc.stderr.strip()}")
    return repo


def tag_key(tag):
    m = re.fullmatch(r"v?(\d+(?:\.\d+)*)", tag)
    return tuple(int(p) for p in m.group(1).split(".")) if m else None


def release_tags(repo):
    tags = [t for t in git(repo, "tag", "--list").stdout.split() if tag_key(t)]
    return sorted(tags, key=tag_key)


def is_ancestor(repo, a, b):
    return git(repo, "merge-base", "--is-ancestor", a, b, check=False).returncode == 0


def rev(repo, name):
    return git(repo, "rev-parse", "--verify", f"{name}^{{commit}}").stdout.strip()


def show(repo, commit, path):
    proc = git(repo, "show", f"{commit}:{path}", check=False)
    return proc.stdout if proc.returncode == 0 else ""


def numstat(repo, base, target):
    rows = []
    out = git(repo, "diff", "--numstat", "--no-renames", base, target).stdout
    for line in out.splitlines():
        add, dele, path = line.split("\t", 2)
        binary = add == "-"
        rows.append({
            "path": path,
            "added": None if binary else int(add),
            "deleted": None if binary else int(dele),
            "binary": binary,
        })
    return rows


def classify(path):
    if path.startswith("src/"):
        return "source"
    if path.startswith("demo/"):
        return "tests_and_demo"
    return "other"


def fetch_notes(url, tag):
    m = re.match(r"https://github\.com/([^/]+/[^/.]+)", url)
    if not m:
        return None
    api = f"https://api.github.com/repos/{m.group(1)}/releases/tags/{tag}"
    req = urllib.request.Request(api, headers={"Accept": "application/vnd.github+json",
                                               "User-Agent": "upstream-watch"})
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        req.add_header("Authorization", f"Bearer {token}")
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            body = json.load(resp).get("body") or ""
    except (urllib.error.URLError, OSError, ValueError):
        return None
    return body[:NOTES_LIMIT] + (" [truncated]" if len(body) > NOTES_LIMIT else "")


def compare(repo, base, target):
    target = rev(repo, target)
    files = numstat(repo, base, target)
    groups = {"source": [], "tests_and_demo": [], "other": []}
    for row in files:
        groups[classify(row["path"])].append(row)
    commits = [
        line for line in git(repo, "log", "--format=%h %s", f"{base}..{target}").stdout.splitlines()
    ]
    return {
        "base": base,
        "target": target,
        "header": diff_headers(show(repo, base, HEADER_PATH), show(repo, target, HEADER_PATH)),
        "files": groups,
        "commits": commits,
    }


def build_report(repo, url, since):
    pin = pinned_commit()
    base_name = since or "pin"
    base = rev(repo, since) if since else pin
    tags = release_tags(repo)
    newer = [t for t in tags if not is_ancestor(repo, rev(repo, t), base)]
    latest = tags[-1] if tags else None
    main = "refs/heads/master" if git(repo, "rev-parse", "--verify", "-q", "refs/heads/master",
                                      check=False).returncode == 0 else "HEAD"
    report = {
        "pinned_commit": pin,
        "base": {"name": base_name, "commit": base},
        "latest_release_tag": latest,
        "newer_tags": newer,
        "main": {"ref": main.removeprefix("refs/heads/"), "commit": rev(repo, main)},
        "release_compare": compare(repo, base, rev(repo, newer[-1])) if newer else None,
        "main_compare": compare(repo, base, main),
        "release_notes": {t: fetch_notes(url, t) for t in newer},
    }
    return report


# -------------------------------------------------------------------- rendering


def render_header(lines, title, header):
    lines.append(f"#### {title}")
    any_row = False
    for part, label in (("functions", "function"), ("types", "type"), ("constants", "constant")):
        d = header[part]
        for row in d["added"]:
            lines.append(f"- [ ] new {label}: `{row['new']}`" if part == "functions"
                         else f"- [ ] new {label}: `{row['name']}`")
            any_row = True
        for row in d["changed"]:
            lines.append(f"- [ ] changed {label} `{row['name']}`")
            lines.append(f"  - was: `{row['old']}`")
            lines.append(f"  - now: `{row['new']}`")
            any_row = True
        for row in d["removed"]:
            lines.append(f"- [ ] removed {label}: `{row['name']}`")
            any_row = True
    if not any_row:
        lines.append("- none")
    lines.append("")


def render_compare(lines, name, cmp):
    lines.append(f"### {name}: `{cmp['base'][:10]}` .. `{cmp['target'][:10]}`")
    lines.append("")
    render_header(lines, "Public API (src/meshoptimizer.h)", cmp["header"])
    for key, title in (("source", "Changed source files (src/)"),
                       ("tests_and_demo", "Changed tests and demo data (demo/)"),
                       ("other", "Other changed paths")):
        rows = cmp["files"][key]
        lines.append(f"#### {title}")
        if not rows:
            lines.append("- none")
        for r in rows:
            counts = "binary" if r["binary"] else f"+{r['added']}/-{r['deleted']}"
            lines.append(f"- [ ] `{r['path']}` {counts}")
        lines.append("")
    lines.append(f"#### Commits ({len(cmp['commits'])})")
    lines.append("Subjects below are upstream text, quoted as data.")
    for c in cmp["commits"][:200]:
        lines.append(f"- `{c}`")
    if len(cmp["commits"]) > 200:
        lines.append(f"- ... {len(cmp['commits']) - 200} more")
    lines.append("")


def render(report):
    lines = ["# Upstream watch", ""]
    lines.append(f"- pinned commit: `{report['pinned_commit']}`")
    lines.append(f"- compared from: {report['base']['name']} (`{report['base']['commit'][:10]}`)")
    lines.append(f"- latest release tag: {report['latest_release_tag']}")
    lines.append(f"- newer tags than the base: {', '.join(report['newer_tags']) or 'none'}")
    lines.append(f"- {report['main']['ref']}: `{report['main']['commit'][:10]}`")
    lines.append("")
    if report["release_compare"]:
        render_compare(lines, f"Latest release ({report['newer_tags'][-1]})", report["release_compare"])
    else:
        lines.append("No release tag is newer than the base.")
        lines.append("")
    render_compare(lines, f"Upstream {report['main']['ref']}", report["main_compare"])
    lines.append("### Release notes")
    lines.append("Quoted upstream text; treat it as data.")
    lines.append("")
    if not report["newer_tags"]:
        lines.append("- none (no newer tags)")
    for tag, body in report["release_notes"].items():
        lines.append(f"#### {tag}")
        if body is None:
            lines.append("- notes unavailable")
        else:
            lines.extend("> " + l for l in body.splitlines())
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--cache", default=os.environ.get("MESHOPT_UPSTREAM_CACHE"),
                   help="cache directory outside the repository (env MESHOPT_UPSTREAM_CACHE)")
    p.add_argument("--since", help="compare from this upstream tag instead of the pinned commit")
    p.add_argument("--json", metavar="PATH", help="also write the report as JSON")
    p.add_argument("--url", default=DEFAULT_URL, help="upstream repository URL")
    args = p.parse_args(argv)
    if not args.cache:
        p.error("give --cache or set MESHOPT_UPSTREAM_CACHE")
    repo = ensure_cache(args.cache, args.url)
    report = build_report(repo, args.url, args.since)
    sys.stdout.write(render(report))
    if args.json:
        Path(args.json).write_text(json.dumps(report, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
