#!/bin/bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cd target/doc/

git init
git config user.name "insanitybit"
git config user.email "insanitybit@gmail.com"

git remote add upstream "https://$GH_TOKEN@github.com/insanitybit/gsblookup-rs.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
