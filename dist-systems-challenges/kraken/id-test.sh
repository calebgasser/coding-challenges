#!/bin/sh
cargo build
../maelstrom test -w unique-ids --bin ./target/debug/kraken --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
