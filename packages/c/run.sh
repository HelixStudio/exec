# /bin/sh

OUT=./a.out

clang --std=c17 $1 -o $OUT

$OUT

rm $OUT
