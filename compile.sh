#!/bin/bash

llc -o out.s $1
# clang -L c/out/ -l foo -o test_run out.s
clang -o test_run out.s
rm out.s