#!/bin/bash

./target/debug/llvm-js-compiler --input=$1 --output=$2
llc -o out.s $2
clang++ -L build/lib/ -o test_run out.s -l llvm-js -l fmt
rm out.s