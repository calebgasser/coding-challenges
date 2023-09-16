#!/bin/sh
go build .
../maelstrom test -w echo --bin ./dist-go --node-count 1 --time-limit 10
