RUSTC ?= rustc
RUSTFLAGS ?= -O -Z debug-info
VERSION=0.1-pre
LIB=urlparse

lib_so=build/lib-$(VERSION).so
lib_files=\
	      $(wildcard src/*.rs)
$examples_files=\
	      $(wildcard src/examples/*.rs)

$(lib_so): $(lib_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs --out-dir=build

$(examples_so): $(examples_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/examples/main.rs --out-dir=build

all: $(lib_so) examples

src/codegen/codegen: $(wildcard src/codegen/*.rs)
	$(RUSTC) $(RUSTFLAGS) $@.rs

src/generated/%.rs: src/codegen/codegen
	src/codegen/codegen $(patsubst src/generated/%,%,$@) src/generated/

build/%:: src/%.rs src/examples/%.rs $(lib_so)
	mkdir -p '$(dir $@)'
	$(RUSTC) $(RUSTFLAGS) $< -o $@ -out-dir=build/

examples: $(patsubst src/examples/%.rs,build/examples/%,$(wildcard src/examples/*/*.rs))

build/tests: $(lib_files)
	$(RUSTC) $(RUSTFLAGS) --test -o build/tests src/lib.rs

build/quicktests: $(lib_files)
	$(RUSTC) --test -o build/quicktests src/lib.rs

# Can't wait for everything to build, optimised too? OK, you can save some time here.
quickcheck: build/quicktests
	build/quicktests --test

check: clean all build/tests
	build/tests --test

test:check
tests:check

clean:
	rm -rf src/generated/ src/codegen/codegen
	rm -rf build/

.PHONY: all examples clean check
