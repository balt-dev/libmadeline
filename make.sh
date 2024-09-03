#!/usr/bin/env sh
cargo build --release

rm -rf dist
mkdir dist

cbindgen > dist/libmadeline.h
cp target/release/libmadeline.dll dist
cp target/release/libmadeline.pdb dist
cp dist/* demo