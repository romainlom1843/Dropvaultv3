#!/bin/sh
sleep 5
cp /tmp/Cargo-tools/Cargo.toml /tmp/Cargo.toml
diesel setup
diesel migration run
