#!/bin/sh
cargo build 
../maelstrom test -w echo --bin ./target/debug/kraken --node-count 1 --time-limit 10
