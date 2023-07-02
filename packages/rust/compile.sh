# /bin/sh

SRC=$1
OUT=$2

if [ -n $3 ] 
then
    OPT=$3
else
    OPT=--crate-type bin
fi

rustc $OPT $SRC -o $OUT
