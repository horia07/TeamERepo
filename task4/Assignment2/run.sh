#!/usr/bin/env bash

####  Compile runtime library  ############

set -e
cd runtime_lib
bash ./create_runtimelib.sh
cd ..

#### Compile llvm pass 

cd ./llvmpasses
make
cd ../

### Instrument a test program with instrumentation

make clean;
make 

#TESTSRC=./tests

#clang -flto \
#-Xclang -load -Xclang "./llvmpass/SwissBoundsChecker.so"  \
#-Xlinker ./runtime_lib/obj/runtime.o \
#"${TESTSRC}/mymalloc.c" "${TESTSRC}/myfree.c" "${TESTSRC}/mymemcpy.c" "${TESTSRC}/main.c" \
#-o example 

set +e
./mytest
