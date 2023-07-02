# /bin/sh

SRC=$1
OUT=$2

if [ -n $3 ] 
then
    OPT=$3
else
    OPT=-O2
fi

ghc $OPT $SRC -o $OUT
