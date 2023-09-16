#!/bin/sh
go build . 
../maelstrom test -w broadcast --bin ./dist-go --node-count 1 --time-limit 20 --rate 10 