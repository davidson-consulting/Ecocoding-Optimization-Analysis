#!/bin/bash


echo "sum,i_iter,i_joule,i_time,o_iter,o_joule,o_time"
for sum in 12 100 1000 10000 100000 1000000
do
  echo -n "$sum"
  for mode in i o
  do
    mite=`grep iterations "${mode}_${sum}.out" | cut -d" " -f6`
    mtime=`grep real "${mode}_${sum}.time" | cut -d" " -f2`
    mjoule=`grep Process "${mode}_${sum}.vjoule" | cut -d";" -f2 | sed 's/ //g' | sed 's/J$//'`
    echo -n ",$mite,$mjoule,$mtime"
  done
  echo ""
done
