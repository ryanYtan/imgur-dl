#!/bin/bash

PROG=imgur-dl
mkdir -p ~/bin
cargo build --release
cp ./target/release/$PROG ~/bin/$PROG
