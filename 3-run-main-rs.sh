#!/bin/bash
RUSTFLAGS="-C link-arg=-Tlink.x" cargo run
exit $?
