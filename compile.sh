#!/bin/bash

llc -filetype=obj -o out.o $1
clang -o test_run out.o
rm out.o