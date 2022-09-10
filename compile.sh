#!/bin/bash

llc -o out.s $1
clang -L c/out/ -l llvm-js-lib -o test_run out.s
rm out.s