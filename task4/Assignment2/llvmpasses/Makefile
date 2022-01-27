SOURCES = boundschecker_pass.cpp 
LIB_NAME = SwissBoundsChecker.so

LDFLAGS = $(shell llvm-config --ldflags)
CXXFLAGS = -ggdb3 -Wall -fno-rtti -fPIC -shared $(shell llvm-config --cxxflags)

all: $(LIB_NAME)

$(LIB_NAME): $(SOURCES)
	g++ $(CXXFLAGS) $(LDFLAGS) $^ -o $@

clean:
	rm -f $(LIB_NAME)
