#!/bin/bash
set -euo pipefail

DEV_BRANCH="${1:-}"
COMMIT_MSG="${2:-}"

if [[ -z "$DEV_BRANCH" ]]; then
  echo "Usage: $0 <dev-branch> [commit message]" >&2
  exit 2
fi

if [[ -z "$COMMIT_MSG" ]]; then
  COMMIT_MSG="saphyr-parser: ${DEV_BRANCH}"
fi

git checkout master
git fetch ethiraric

git checkout dev/saphyr-parser
git rebase master

git merge --squash "${DEV_BRANCH}"
git commit -m "$COMMIT_MSG"

git log | grep bw_



