#!/bin/bash
cargo watch -q -c -s 'wasm-pack build --target nodejs --dev && ts-node ./tests/node.ts' -w src