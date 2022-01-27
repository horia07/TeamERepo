# Compiler assignment

1. You can use the provided `default.nix` to get all build tools by running `nix-shell`
   ```
   $ nix-shell
   ```
2. Afterwards you can build and run the example using
   ```console
   $ ./run.sh
   ```
   “run.sh” performs following:
        
        1) Builds runtime library
            - By running /ROOT/runtime_lib/create_runtimelib.sh
        2) Builds llvm pass
            - By running /ROOT/llvmpass/Makefile
        3) Builds/instruments a test program
            - By running /ROOT/Makefile
            (Test source codes are placed in /ROOT/tests/)
        4) Runs an executable
            - By running /ROOT/mytest

    Each script above can be performed independently

