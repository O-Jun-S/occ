#!/bin/bash

occ="./target/debug/occ"

assert() {
    expected="$1"
    input="$2"

    ${occ} "$input" > tmp.s
    gcc -static -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" == "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input expected, but got $actual"
        exit 1
    fi
}

cargo build

assert 0 0
assert 42 42
assert 9 "3*3*3/3"
assert 23 "13+9+9/9"

echo The test ended correctly.

