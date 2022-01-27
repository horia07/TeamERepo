TARGET   = mytest 

# CHECK THE PATH! AND MODIFY ME! from here  ##################

RUNTIMELIB = ./runtime_lib/
RUNTIMEOBJ = $(RUNTIMELIB)/obj/runtime.o
SRCDIR = ./tests/
OBJDIR = ./tests/obj/
BOUNDSCHECK_PASS = ./llvmpasses/SwissBoundsChecker.so

CC = clang
CXX = clang++
LLC = llc 
LLVMLINK = llvm-link
OPT = opt

OPTLEVEL = -O1
CFLAGS = -flto -Xclang -load -Xclang $(BOUNDSCHECK_PASS) 
LDFLAGS = -lm

SOURCES  := $(wildcard $(SRCDIR)/*.c)
OBJECTS  := $(SOURCES:$(SRCDIR)/%.c=$(OBJDIR)/%.o)
HOOKED   = ./tests/obj/hooked.o  
LLVMLINKED   = ./tests/obj/llvmlinked.o  

$(TARGET) : $(SOURCES)
	@mkdir -p $(@D)
	clang $(CFLAGS) $(LDFLAGS) -Xlinker $(RUNTIMEOBJ) $(SOURCES) -o $@ 
	@echo "-- Compiling "$<" complete"

.PHONY: clean
clean:
	rm -f $(OBJECTS) mytest


