git clone https://github.com/llvm/llvm-project.git
cd llvm-project 
cmake -S llvm -B build -G "Ninja"
cmake --build build --target opt
./build/bin/opt --version 


cd ..
ln -s $PWD/DeadCodeElimination.cpp $PWD/llvm-project/llvm/lib/Transforms/Utils/DeadCodeElimination.cpp -f
ln -s $PWD/DeadCodeElimination.h $PWD/llvm-project/llvm/include/llvm/Transforms/Utils/DeadCodeElimination.h -f
ln -s $PWD/CMakeLists.txt $PWD/llvm-project/llvm/lib/Transforms/Utils/CMakeLists.txt -f
ln -s $PWD/PassRegistry.def $PWD/llvm-project/llvm/lib/Passes/PassRegistry.def -f
ln -s $PWD/PassBuilder.cpp $PWD/llvm-project/llvm/lib/Passes/PassBuilder.cpp -f

echo linked files