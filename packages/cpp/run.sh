# /bin/sh

OUT=./a.out

clang++ --std=c++20 $1 -o $OUT

$OUT

rm $OUT
