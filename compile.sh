#!/bin/bash

llc -o out.s $1
clang++ -L build/lib/ -l llvm-js -l fmt -o test_run out.s
rm out.s