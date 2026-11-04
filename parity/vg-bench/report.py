"""Turn the harness JSON lines into markdown tables.

    python3 -I parity/vg-bench/report.py <work-dir>/<mesh>.jsonl ...
"""
import json
import statistics
import sys


def fmt(v, digits=2):
    if v is None:
        return "n/a"
    if isinstance(v, float):
        return f"{v:,.{digits}f}"
    return f"{v:,}"


def p95(values):
    values = sorted(values)
    return values[min(len(values) - 1, int(round((len(values) - 1) * 0.95 + 0.4999)))]


def interpolate(points, target):
    """Triangles at `target` deviation from sorted (deviation, triangles) points, log-log."""
    import math

    for (d0, t0), (d1, t1) in zip(points, points[1:]):
        if d0 <= target <= d1 and d1 > d0:
            w = (math.log(target) - math.log(d0)) / (math.log(d1) - math.log(d0))
            return math.exp(math.log(t0) + w * (math.log(t1) - math.log(t0)))
    return None


def load(paths):
    rows = []
    for path in paths:
        with open(path) as f:
            rows += [json.loads(line) for line in f if line.startswith("{")]
    return rows


def main(paths):
    rows = load(paths)
    meshes = sorted({r["mesh"] for r in rows if "mesh" in r})
    for mesh in meshes:
        mine = [r for r in rows if r.get("mesh") == mesh]
        builds = [r for r in mine if r["kind"] == "build"]
        tris = builds[0]["triangles"] if builds else 0
        print(f"### {mesh} ({tris:,} source triangles)\n")
        print("| Builder | Threads | Build ms median | p95 | noise (max-min) | Serialise ms median | Peak RSS MiB median | Repeats |")
        print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
        for key in sorted({(r["builder"], r["threads"]) for r in builds}):
            g = [r for r in builds if (r["builder"], r["threads"]) == key]
            t = [r["build_ms"] for r in g]
            print(
                f"| {key[0]} | {key[1]} | {fmt(statistics.median(t), 1)} | {fmt(p95(t), 1)} | {fmt(max(t) - min(t), 1)} | "
                f"{fmt(statistics.median(r['serialise_ms'] for r in g), 1)} | "
                f"{fmt(statistics.median(r['rss_kib'] for r in g) / 1024, 0)} | {len(g)} |"
            )
        shapes = {r["builder"]: r for r in mine if r["kind"] == "shape"}
        if shapes:
            print("\n| Metric | " + " | ".join(shapes) + " |")
            print("| --- | " + " | ".join("---:" for _ in shapes) + " |")
            keys = [
                ("clusters", "clusters"),
                ("levels", "levels"),
                ("hierarchy_depth", "hierarchy depth"),
                ("total_over_leaf", "total / leaf triangles"),
                ("tris_per_cluster_mean", "triangles per cluster, mean"),
                ("tris_per_cluster_p5", "triangles per cluster, p5"),
                ("verts_per_cluster_mean", "vertices per cluster, mean"),
                ("verts_per_cluster_p5", "vertices per cluster, p5"),
                ("pages", "pages"),
                ("page_fill_mean", "mean page fill"),
                ("stuck_groups", "stuck groups"),
                ("file_bytes", "file bytes"),
                ("file_zstd19_bytes", "file bytes after zstd -19"),
                ("decompressed_zstd19_bytes", "decompressed stream after zstd -19"),
                ("resident_bytes", "resident bytes"),
                ("decode_mb_per_s", "whole-asset decode MB/s"),
                ("page_decode_mb_per_s", "page decode MB/s"),
            ]
            for key, name in keys:
                if any(key in s for s in shapes.values()):
                    print(f"| {name} | " + " | ".join(fmt(s.get(key), 3 if key == "page_fill_mean" else 2) for s in shapes.values()) + " |")
            for name, s in shapes.items():
                total = s["total_triangles"]
                print(f"\n{name}: {s['resident_bytes'] / total:.1f} resident B per DAG triangle, "
                      f"{s['resident_bytes'] / s['source_triangles']:.1f} per source triangle; "
                      f"file {s['file_bytes'] / total:.1f} B per DAG triangle.")
        checks = [r for r in mine if r["kind"] == "check"]
        if checks:
            print("\n| Builder | Self-check | Result | Detail |")
            print("| --- | --- | --- | --- |")
            for r in checks:
                print(f"| {r['builder']} | {r['name']} | {'pass' if r['pass'] else 'FAIL'} | {r['detail']} |")
        cuts = [r for r in mine if r["kind"] == "cut"]
        if cuts:
            print("\nCut quality over all spans (mean triangles; p95 of per-view maximum deviation in pixels; measured/claimed p95):\n")
            print("| Builder | Threshold px | Mean triangles | p95 max dev px | mean max dev px | p95 rms dev px | measured/claimed median | p95 | nodes | clusters tested | clusters drawn |")
            print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
            for r in cuts:
                if r["span_px"] != -1.0:
                    continue
                threshold = "0 (source)" if r["threshold_px"] < 1e-6 else r["threshold_px"]
                print(
                    f"| {r['builder']} | {threshold} | {fmt(r['mean_triangles'], 0)} | {fmt(r['p95_max_dev_px'])} | "
                    f"{fmt(r['mean_max_dev_px'])} | {fmt(r['p95_rms_dev_px'])} | {fmt(r['median_measured_over_claimed'])} | "
                    f"{fmt(r['p95_measured_over_claimed'])} | {fmt(r['mean_nodes'], 1)} | {fmt(r['mean_clusters_tested'], 0)} | "
                    f"{fmt(r['mean_clusters_drawn'], 0)} |"
                )
        curves = {}
        for r in cuts:
            if r["span_px"] == -1.0 and r["threshold_px"] >= 1e-6 and r["p95_max_dev_px"]:
                curves.setdefault(r["builder"], []).append((r["p95_max_dev_px"], r["mean_triangles"]))
        if len(curves) == 2:
            print("\nMean cut triangles at equal p95 maximum deviation (log-log interpolation along the four thresholds; n/a outside the range of either curve):\n")
            names = sorted(curves)
            print("| p95 deviation px | " + " | ".join(f"{n} triangles" for n in names) + " | ratio " + "/".join(names) + " |")
            print("| ---: | " + " | ".join("---:" for _ in names) + " | ---: |")
            for target in (3, 5, 8):
                cells = [interpolate(sorted(curves[n]), target) for n in names]
                ratio = cells[0] / cells[1] if all(cells) else None
                print(f"| {target} | " + " | ".join(fmt(c, 0) if c else "n/a" for c in cells) + f" | {fmt(ratio, 2) if ratio else 'n/a'} |")
        if PARETO and cuts:
            print("\nPer span (mean triangles, p95 max deviation px):\n")
            names = sorted({r["builder"] for r in cuts})
            print("| Span px | Threshold px | " + " | ".join(f"{n} triangles | {n} p95 dev" for n in names) + " |")
            print("| ---: | ---: | " + " | ".join("---: | ---:" for _ in names) + " |")
            keyed = {(r["builder"], r["span_px"], r["threshold_px"]): r for r in cuts}
            for span in sorted({r["span_px"] for r in cuts if r["span_px"] > 0}):
                for thr in sorted({r["threshold_px"] for r in cuts}):
                    cells = []
                    for n in names:
                        r = keyed.get((n, span, thr))
                        cells += [fmt(r["mean_triangles"], 0), fmt(r["p95_max_dev_px"])] if r else ["n/a", "n/a"]
                    print(f"| {int(span)} | {'0' if thr < 1e-6 else thr} | " + " | ".join(cells) + " |")
        print()


def summary(paths):
    """One compact table per topic across meshes (the numbers quoted in RESULTS.md)."""
    rows = load(paths)
    meshes = sorted({r["mesh"] for r in rows if "mesh" in r}, key=lambda m: next(r["triangles"] for r in rows if r.get("mesh") == m and r["kind"] == "build"))

    def median_build(mesh, builder, threads, key="build_ms"):
        g = [r["build_ms"] + r["serialise_ms"] if key == "build_ms" else r["rss_kib"] / 1024 for r in rows if r.get("mesh") == mesh and r["kind"] == "build" and r["builder"] == builder and r["threads"] == threads]
        return statistics.median(g), max(g) - min(g)

    threads = max(r["threads"] for r in rows if r["kind"] == "build")
    print("Build plus serialise time (ours: DAG, hierarchy and MDAG 1 write; bevy: from_mesh and save), median of interleaved repeats (noise = max - min), milliseconds:\n")
    print(f"| Mesh | Triangles | ours 1T | bevy 1T | ours/bevy | ours {threads}T | bevy {threads}T | ours/bevy | ours RSS MiB ({threads}T) | bevy RSS MiB |")
    print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    for m in meshes:
        tris = next(r["triangles"] for r in rows if r.get("mesh") == m and r["kind"] == "build")
        cells = []
        for t in (1, threads):
            o, ob = median_build(m, "ours", t), median_build(m, "bevy", t)
            cells += [f"{fmt(o[0], 0)} (+-{fmt(o[1], 0)})", f"{fmt(ob[0], 0)} (+-{fmt(ob[1], 0)})", f"{o[0] / ob[0]:.2f}"]
        ro, rb = median_build(m, "ours", threads, "rss_mib")[0], median_build(m, "bevy", threads, "rss_mib")[0]
        print(f"| {m} | {tris:,} | " + " | ".join(cells) + f" | {ro:,.0f} | {rb:,.0f} |")

    shapes = {(r["mesh"], r["builder"]): r for r in rows if r["kind"] == "shape"}
    print("\nSize and shape (bytes per DAG triangle use total triangles over all levels):\n")
    print("| Mesh | file B/tri ours | bevy | file zstd-19 B/tri ours | bevy | resident B/tri ours | bevy | total/leaf ours | bevy | tris/cluster mean ours | bevy | p5 ours | bevy | hierarchy depth ours | bevy |")
    print("| --- | " + " | ".join("---:" for _ in range(14)) + " |")
    for m in meshes:
        o, b = shapes[(m, "ours")], shapes[(m, "bevy")]
        per = lambda s, k: s[k] / s["total_triangles"]
        print(
            f"| {m} | {per(o, 'file_bytes'):.1f} | {per(b, 'file_bytes'):.1f} | {per(o, 'file_zstd19_bytes'):.1f} | {per(b, 'file_zstd19_bytes'):.1f} | "
            f"{per(o, 'resident_bytes'):.1f} | {per(b, 'resident_bytes'):.1f} | {o['total_over_leaf']:.2f} | {b['total_over_leaf']:.2f} | "
            f"{o['tris_per_cluster_mean']:.1f} | {b['tris_per_cluster_mean']:.1f} | {o['tris_per_cluster_p5']:.0f} | {b['tris_per_cluster_p5']:.0f} | "
            f"{o['hierarchy_depth']} | {b['hierarchy_depth']} |"
        )

    print("\nCut efficiency and traversal (all 64 directions x 6 spans; 1 px threshold row for traversal):\n")
    print("| Mesh | tris ours/bevy at p95 dev 5 px | at 8 px | measured/claimed median ours | bevy | nodes ours | bevy | clusters tested ours | bevy | drawn ours | bevy | quantisation floor p95 px (bevy, source cut) |")
    print("| --- | " + " | ".join("---:" for _ in range(11)) + " |")
    for m in meshes:
        cuts = [r for r in rows if r.get("mesh") == m and r["kind"] == "cut" and r["span_px"] == -1.0]
        curves = {b: sorted((r["p95_max_dev_px"], r["mean_triangles"]) for r in cuts if r["builder"] == b and r["threshold_px"] >= 1e-6) for b in ("ours", "bevy")}
        ratios = []
        for target in (5, 8):
            a, b = interpolate(curves["ours"], target), interpolate(curves["bevy"], target)
            ratios.append(f"{a / b:.2f}" if a and b else "n/a")
        at = lambda b, k: next(r[k] for r in cuts if r["builder"] == b and r["threshold_px"] == 1.0)
        floor = next(r["p95_max_dev_px"] for r in cuts if r["builder"] == "bevy" and r["threshold_px"] < 1e-6)
        print(
            f"| {m} | {ratios[0]} | {ratios[1]} | {fmt(at('ours', 'median_measured_over_claimed'))} | {fmt(at('bevy', 'median_measured_over_claimed'))} | "
            f"{fmt(at('ours', 'mean_nodes'), 1)} | {fmt(at('bevy', 'mean_nodes'), 1)} | {fmt(at('ours', 'mean_clusters_tested'), 0)} | {fmt(at('bevy', 'mean_clusters_tested'), 0)} | "
            f"{fmt(at('ours', 'mean_clusters_drawn'), 0)} | {fmt(at('bevy', 'mean_clusters_drawn'), 0)} | {fmt(floor)} |"
        )
    bad = [r for r in rows if r["kind"] == "check" and not r["pass"]]
    print(f"\nSelf-checks: {sum(1 for r in rows if r['kind'] == 'check')} run, {len(bad)} failed.")


PARETO = "--pareto" in sys.argv

if __name__ == "__main__":
    files = [a for a in sys.argv[1:] if not a.startswith("--")]
    summary(files) if "--summary" in sys.argv else main(files)
