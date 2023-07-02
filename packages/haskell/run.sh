# /bin/sh

OUT=./a.out

ghc $1 -v0 -o $OUT

$OUT

rm ./packages/haskell/test.hi ./packages/haskell/test.o ./a.out
