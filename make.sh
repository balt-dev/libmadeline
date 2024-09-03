#!/usr/bin/env sh
cargo build --release

rm -rf dist
mkdir dist

cbindgen > dist/libmadeline.h
cp target/release/libmadeline.* dist
cp dist/* demo