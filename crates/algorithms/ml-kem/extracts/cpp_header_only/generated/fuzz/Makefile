CC=clang-18
CXX=clang++-18
FUZZ_FLAGS=-fsanitize=address,undefined,fuzzer -fno-omit-frame-pointer
FUZZ_LIBS=-L/usr/lib/llvm-18/lib -lFuzzer

CXXFLAGS=-O2 -g -Wall -Wextra -Wno-unused-parameter -Wno-exceptions -I..
CFLAGS=$(CXXFLAGS)
LDFLAGS=-g
LIBS=
COMMON_DEPS=

TARGETS=enc_fuzz valid_fuzz dec_fuzz mkcorpus

all: $(TARGETS)

.cc.o:
	$(CXX) $(CXXFLAGS) $(FUZZ_FLAGS) -c $< -o $@

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@

enc_fuzz: enc_fuzz.o $(COMMON_DEPS)
	$(CXX) -o $@ enc_fuzz.o $(LDFLAGS) $(LIBS) $(FUZZ_FLAGS) $(FUZZ_LIBS)

dec_fuzz: dec_fuzz.o $(COMMON_DEPS)
	$(CXX) -o $@ dec_fuzz.o $(LDFLAGS) $(LIBS) $(FUZZ_FLAGS) $(FUZZ_LIBS)

valid_fuzz: valid_fuzz.o $(COMMON_DEPS)
	$(CXX) -o $@ valid_fuzz.o $(LDFLAGS) $(LIBS) $(FUZZ_FLAGS) $(FUZZ_LIBS)

mkcorpus: mkcorpus.o
	$(CC) -o $@ mkcorpus.o

clean:
	-rm -f *.o $(TARGETS)
