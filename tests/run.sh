#!/bin/bash
cd $(dirname $0)/..

echo "ENV SETUP"
./tests/setup_gpg.sh

echo "RUNNING TESTS"
RUST_BACKTRACE=1 cargo test --verbose
