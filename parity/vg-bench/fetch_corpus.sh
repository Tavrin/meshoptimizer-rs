#!/usr/bin/env bash
# Fetch the public benchmark corpus into a directory outside the repository.
#
#   bash parity/vg-bench/fetch_corpus.sh <corpus-dir>      (or set VG_CORPUS)
#
# Nothing is redistributed: files are downloaded from their publishers and
# verified against parity/vg-bench/corpus.sha256 when an entry exists. Licences
# are recorded in parity/vg-bench/CORPUS.md and copied next to the data.
# Downloaded content is data only; nothing from it is executed.
set -euo pipefail

dest="${1:-${VG_CORPUS:-}}"
if [ -z "$dest" ]; then
  echo "usage: fetch_corpus.sh <corpus-dir>" >&2
  exit 2
fi
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$dest"
cd "$dest"
ua='Mozilla/5.0 (X11; Linux x86_64) Gecko/20100101 Firefox/130.0'
get() { curl -fsSL -A "$ua" --retry 3 -o "$2" "$1"; }

# Stanford 3D Scanning Repository (research use, credit required, no commercial
# use of the models or images without permission). Archives are extracted to
# one PLY per model.
stanford() { # name url member-glob-in-archive
  local name="$1" url="$2" member="$3"
  [ -s "$name.ply" ] && return 0
  get "$url" "$name.tar.gz"
  tar -xzf "$name.tar.gz" --wildcards --strip-components=0 -O "$member" >"$name.ply"
  rm -f "$name.tar.gz"
}
stanford bunny https://graphics.stanford.edu/pub/3Dscanrep/bunny.tar.gz 'bunny/reconstruction/bun_zipper.ply'
stanford dragon https://graphics.stanford.edu/pub/3Dscanrep/dragon/dragon_recon.tar.gz 'dragon_recon/dragon_vrip.ply'
stanford lucy https://graphics.stanford.edu/data/3Dscanrep/lucy.tar.gz 'lucy.ply'

# Poly Haven models (CC0 1.0). glTF plus its binary buffer; textures are skipped.
polyhaven() { # asset id
  local id="$1"
  [ -s "$id.gltf" ] && [ -s "$id.bin" ] && return 0
  get "https://api.polyhaven.com/files/$id" "$id.files.json"
  # The API response is data: read three URL fields, never execute anything.
  python3 -I - "$id" <<'PY' >"$id.urls"
import json, sys
asset = sys.argv[1]
d = json.load(open(f"{asset}.files.json"))["gltf"]["1k"]["gltf"]
print(d["url"])
for name, item in d["include"].items():
    if name.endswith(".bin"):
        print(item["url"])
PY
  { read -r gltf; read -r bin; } <"$id.urls"
  get "$gltf" "$id.gltf"
  get "$bin" "$id.bin"
  # The glTF names its buffer by a relative uri; point it at the local copy.
  python3 -I - "$id" <<'PY'
import json, sys
asset = sys.argv[1]
doc = json.load(open(f"{asset}.gltf"))
for buf in doc["buffers"]:
    buf["uri"] = f"{asset}.bin"
doc["images"] = []
doc["textures"] = []
for mat in doc.get("materials", []):
    mat.pop("normalTexture", None)
    mat.pop("occlusionTexture", None)
    mat.pop("emissiveTexture", None)
    pbr = mat.get("pbrMetallicRoughness", {})
    pbr.pop("baseColorTexture", None)
    pbr.pop("metallicRoughnessTexture", None)
json.dump(doc, open(f"{asset}.gltf", "w"))
PY
  rm -f "$id.files.json" "$id.urls"
}
polyhaven namaqualand_boulder_02
polyhaven coast_rocks_01
polyhaven dead_tree_trunk_02

# Khronos glTF Sample Assets, FlightHelmet: six materials (multi-material case).
# Per-asset licence: CC0 1.0 for the model files (LICENSE.md in the asset).
khr=https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/FlightHelmet
if [ ! -s flighthelmet.gltf ]; then
  get "$khr/glTF/FlightHelmet.gltf" flighthelmet.gltf
  get "$khr/glTF/FlightHelmet.bin" FlightHelmet.bin
  get "$khr/LICENSE.md" flighthelmet.LICENSE.md
fi

cp "$here/CORPUS.md" CORPUS.md
if [ -f "$here/corpus.sha256" ]; then
  sha256sum -c "$here/corpus.sha256"
else
  echo "no corpus.sha256 yet; current hashes:"
  sha256sum bunny.ply dragon.ply lucy.ply namaqualand_boulder_02.bin coast_rocks_01.bin dead_tree_trunk_02.bin FlightHelmet.bin
fi
