#!/usr/bin/env zsh
set -e

cd /usr/local/bin/
url='https://github.com/rust-lang-nursery/mdBook/releases/download/0.0.21/mdBook-0.0.21-x86_64-unknown-linux-gnu.tar.gz'
curl -L ${url} | tar -Uxvz
cd -

cd /tmp
url='https://github.com/zyedidia/micro/releases/download/v1.3.4/micro-1.3.4-linux64.tar.gz'
curl -L ${url} | tar -Uxvz
cp -vf ./micro-*/micro /usr/local/bin/
cd -

cargo install fd exa

chmod +x /usr/local/bin/*
cd ..
