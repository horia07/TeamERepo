# Assignment 1

Welcome to assignment 1! 

Implement a dead code elimination pass:
> An instruction is considered dead if its result is not used by any other instruction and has no externally observable effect.

We only consider cases within function boundaries. See ```Assignment 1/FunctionWithDeadCode.ll``` for a small piece of IR you can test your implementation with. If it is correct, the function will only consist of the return statement after your pass runs. 

LLVM has a small guide for writing passes, which this exercise is based on: https://llvm.org/docs/WritingAnLLVMNewPMPass.html

Bear in mind that there is a second assignment as well, and having a look around the LLVM codebase for some functions to use might save you a lot of time here :-).

We recommend you follow these steps (these will differ if you're using an IDE / VS Code) in your nix shell:

1. Clone LLVM into this directory (Assignment 1) with the following command ```git clone https://github.com/llvm/llvm-project.git```
2. Change directories with ```cd llvm-project```
3. Configure LLVM with ```cmake -S llvm -B build -G "Ninja"```
4. Build a vanilla version of opt with ```cmake --build build --target opt```
5. Check that it works by running ```./build/bin/opt --version```

Now for adding your passe:
1. Create the following two files, assuming you're currently in the llvm-project folder
```llvm/lib/Transforms/Utils/DeadCodeElimination.cpp```
```llvm/include/llvm/Transforms/Utils/DeadCodeElimination.h```
2. Implement your pass
3. In ```llvm/lib/Transforms/Utils/CMakeLists.txt```, add your passe's source file
3. Go to ```llvm/lib/Passes/PassRegistry.def``` and in the 'FUNCTION_PASS' section add ```FUNCTION_PASS("dead-code-elimination-pass", DeadCodeEliminationPass())````. The first argument determines the passes name when running opt in the command line
4. Go to ```llvm/lib/Passes/PassBuilder.cpp``` and include the header file of your pass
5. Build opt again

For testing, you can run ```./build/bin/opt -disable-output -passes=dead-code-elimination-pass ../FunctionWithDeadCode.ll```

For VS Code users we have also included a settings.json which works with the Nix Environment Selector [(install)](vscode:extension/arrterian.nix-env-selector) and CMake Tools [(install)](vscode:extension/ms-vscode.cmake-tools) Plugins.

Please only submit your passe's header and source files.
