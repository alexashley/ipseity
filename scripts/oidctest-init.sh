#!/usr/bin/env bash

if [[ -d "oidctest" ]]; then
  echo "Skipping oidctest clone."
  exit 0
fi

git clone https://github.com/openid-certification/oidctest.git
cd oidctest
git checkout stable-release-1.1.x