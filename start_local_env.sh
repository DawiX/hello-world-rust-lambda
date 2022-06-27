#!/bin/bash
unset PROFILE
unset RUST_BACKTRACE
export PROFILE="YourProfileHere"
export RUST_BACKTRACE=1

cargo lambda watch
