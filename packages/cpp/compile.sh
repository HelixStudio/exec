# /bin/sh

SRC=$1
OUT=$2

if [ -n $3 ] 
then
    OPT=$3
else
    OPT=-std=c++20 -O2
fi

clang++ $OPT $SRC -o $OUT
