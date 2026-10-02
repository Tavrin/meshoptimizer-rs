#!/usr/bin/env bash
set -euo pipefail
: "${MESHOPT_REFERENCE:?set MESHOPT_REFERENCE to the pinned upstream checkout}"
revision="$(git --no-optional-locks -C "$MESHOPT_REFERENCE" rev-parse HEAD)"
if [[ "$revision" != 4c203430ca565cb59a468a91922c76c208169536 ]]; then
  echo 'The oracle requires meshoptimizer at 4c203430ca565cb59a468a91922c76c208169536.' >&2
  exit 1
fi
if [[ -n "$(git --no-optional-locks -C "$MESHOPT_REFERENCE" status --porcelain --untracked-files=all)" ]]; then
  echo 'The oracle requires an unmodified checkout, including untracked files.' >&2
  exit 1
fi
if [[ -n "$(git --no-optional-locks -C "$MESHOPT_REFERENCE" diff v1.3 HEAD -- src)" ]]; then
  echo 'The oracle source must be identical to v1.3.' >&2
  exit 1
fi
