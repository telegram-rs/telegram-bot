#!/bin/bash
# Author: Steve Klabnik
# github.com/steveklabnik/automatically_update_github_pages_with_travis_example

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cargo doc --all
cd target/doc/

git init
git config user.name "Lukas Kalbertodt"
git config user.email "lukas.kalbertodt@gmail.com"

git remote add upstream "https://${GH_TOKEN}@github.com/telegram-rs/telegram-bot.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
