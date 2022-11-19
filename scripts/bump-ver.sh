#!/usr/bin/env bash

SEMVER=""
TMP=Cargo.tmp

cleanup() {
  rm -f $TMP
}
trap cleanup EXIT

rm -f $TMP
while read -r LINE; do
  if [[ $LINE =~ ^version.*\"(.*)\" ]]; then
    SEMVER=${BASH_REMATCH[1]}
    if [[ $SEMVER =~ ([0-9]+)\.([0-9]+)\.([0-9]+) ]]; then
      echo "current version: $SEMVER"
      major=${BASH_REMATCH[1]}
      minor=${BASH_REMATCH[2]}
      patch=${BASH_REMATCH[3]}
      let patch++
      NEXTVER="$major.$minor.$patch"
      echo "next version:    $NEXTVER"
      echo "version = \"$NEXTVER\"" >>$TMP
    else
      echo "Invalid version number"
      exit 1
    fi
  else
    echo $LINE >>$TMP
  fi
done < <(cat $CARGO_MANIFEST_DIR/Cargo.toml)
mv $TMP $CARGO_MANIFEST_DIR/Cargo.toml

[ -z $NEXTVER ] && exit 0

# at this point, Cargo.toml has been updated.
# next: create a build to update Cargo.lock, create a new commit,
# and tag that commit.

cargo build --release
git add $CARGO_MANIFEST_DIR/Cargo.toml $CARGO_MANIFEST_DIR/Cargo.lock
git commit -m "ver: bump to $NEXTVER"
git tag $NEXTVER

# vim:ft=sh
