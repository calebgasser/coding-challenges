#!/bin/sh
go build .
../maelstrom test -w unique-ids --bin ./dist-go --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
