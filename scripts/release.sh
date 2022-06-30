#!/usr/bin/env bash

# takes the tag as an argument (e.g. v0.1.0)
if [ -n "$1" ]; then
    # update the version
    msg="# managed by release.sh"
    sed -i "" -e "s/^version = .* $msg$/version = \"${1#v}\" $msg/" Cargo.toml
    # will update Cargo.lock
    cargo check

    git add -A && git commit -m "chore(release): prepare for $1"
    # generate a changelog for the tag message
    git tag "$1"
else
    echo "warn: please provide a tag"
    exit 1
fi