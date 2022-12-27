#!/bin/bash
set -e

mkdir -p $1/src

sed -i '' "s/# add here/\"$1\",\n  # add here/g" Cargo.toml

echo "hi" > $1/input.txt

echo "[package]
name = \"$1\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
util = { path = \"../util\" }" > $1/Cargo.toml

echo "use $1::*;
use util::*;

fn main() {
    let s = include_str!(\"../input.txt\");

    time(|| {
        println!(\"Solution: {}\", solve(s));
        println!(\"Bonus: {}\", bonus(s));
    });
}" > $1/src/main.rs

echo "pub fn solve(s: &str) -> usize {
    42
}

pub fn bonus(s: &str) -> usize {
    42
}

#[test]
fn test_solve() {
    let s = \"\";

    assert_eq!(solve(s), 0);
}" > $1/src/lib.rs
