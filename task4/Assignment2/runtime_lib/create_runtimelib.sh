#!/usr/bin/env /bash

####  Compile hooks  ############

CLANG=$(which clang)
CLANGPP=$(which clang++)
GCC=$(which gcc)
AR=$(which llvm-ar)

RUNTIME_LIB=./

rm -f ${RUNTIME_LIB}/obj/*.o ${RUNTIME_LIB}/obj/*.a
mkdir -p ${RUNTIME_LIB}/obj

$CLANG -emit-llvm \
${RUNTIME_LIB}/src/runtime.c \
-c -o \
${RUNTIME_LIB}/obj/runtime.o 

