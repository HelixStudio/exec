# /bin/sh

OUT=./a.out

rustc $1 --crate-type bin -o $OUT

$OUT

rm $OUT
