#!/bin/bash
a=1
for i in *; do
  new=$(printf "%03d" "$a") #04 pad to length of 3
  mv -i -- "$i" "$new"
  let a=a+1
done
