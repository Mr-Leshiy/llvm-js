#!/bin/bash

./target/debug/llvm-js-compiler --input=$1 --output=$2
llc -o out.s $2
clang++ -L build/lib/ -l llvm-js -l fmt -o test_run out.s
rm out.s