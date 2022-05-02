#!/bin/bash
HOME="$(cd `dirname "${BASH_SOURCE-$0}"`/..; pwd)"
cd ${HOME}/docker

mkdir temp
cp -r ../migration temp/
cp -r ../src temp/
cp ../.env temp/
cp ../Cargo.toml temp/
cp ../Cargo.lock temp/

docker build . -t luban

rm -rf temp
cd ${HOME}
