#!/bin/bash
wasm-pack build --release -t nodejs
wasm-pack pack
wasm-pack publish