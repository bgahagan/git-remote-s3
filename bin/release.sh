#!/bin/bash

set -e

changelog() {
  head="${1:-HEAD}"

  for sha in `git rev-list -n 100 --first-parent "$head"^`; do
    previous_tag="$(git tag -l --points-at "$sha" 'v*' 2>/dev/null || true)"
    [ -z "$previous_tag" ] || break
  done

  if [ -z "$previous_tag" ]; then
    echo "Couldn't detect previous version tag" >&2
    exit 1
  fi

  git log --no-merges --format='%C(auto,green)* %s%C(auto,reset)%n%w(0,2,2)%+b' \
    --reverse "${previous_tag}..${head}"
}

stage_github() {

  project_name="${1?}"
  version="${2?}"
  [[ $version == *-* ]] && pre=1 || pre=

  assets=()
  while read -r filename label; do
    assets+=( -a "${filename}#${label}" )
  done

  if hub release --include-drafts | grep -q "^v${version}\$"; then
    hub release edit "v${version}" -m "" "${assets[@]}"
  else
    { echo "${project_name} v${version}"
      echo
      changelog
    } | hub release create --draft ${pre:+--prerelease} -F - "v${version}" "${assets[@]}"
  fi
}

build() {
  for target in x86_64-unknown-linux-gnu i686-unknown-linux-gnu; do
    cargo build --release --target "$target"
    version=$(egrep "^version\s+=" Cargo.toml | egrep -o "[0-9]+\.[0-9]+\.[0-9]+")
    gzip -c target/$target/release/git-remote-s3 > target/$target/release/git-remote-s3-$target.gz
    echo "target/$target/release/git-remote-s3-$target.gz git-remote-s3-$target.gz" | \
      stage_github "git-remote-s3" "$version"
  done
  echo "Finished building and staging binaries.  Manual next stpes:"
  echo " - Promote github relase"
  echo " - publish crate: cargo publish"
}

build
