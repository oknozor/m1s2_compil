#!/bin/bash
cargo build
cp target/debug/rjsc ../sandbox
cd sandbox && make all