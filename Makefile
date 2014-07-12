RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/grunge.rs
TEST_FILE           = $(SRC_DIR)/test.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

LIB_DIR             = target
DEPS_DIR            = $(LIB_DIR)/deps
TEST_DIR            = $(LIB_DIR)/test
EXAMPLE_DIR         = $(LIB_DIR)/examples

DOC_DIR             = doc
DOC_PARAMS          = -L $(DEPS_DIR) --html-in-header src/docs/mathjax.html

.PHONY: all lib test bench check doc clean help

all: lib doc

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(TEST_DIR)
	rm -rf $(BENCH_DIR)
	rm -rf $(DOC_DIR)

help:
	@echo "--- grunge"
	@echo "make             - Build the library & documentation."
	@echo "make lib         - Build the library."
	@echo "make test        - Run the unit tests."
	@echo "make bench       - Run benchmarks."
	@echo "make doc         - Builds the library's documentation."
	@echo "make examples    - Builds the examples."
	@echo "make clean       - Removes all generated files."

# Library

lib: $(LIB_FILE)
	mkdir -p $(LIB_DIR)
	rm -f $(LIB_DIR)/libgrunge*.rlib  # Quick fix for using Cargo + make together
	$(RUSTC) -L $(DEPS_DIR) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

# Testing and Benchmarking

test: lib
	mkdir -p $(TEST_DIR)
	$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(TEST_DIR) --test $(TEST_FILE)
	$(TEST_DIR)/test

bench: test
	$(TEST_DIR)/test --bench

# Documentation

doc:
	mkdir -p $(DOC_DIR)
	$(RUSTDOC) $(DOC_PARAMS) -o $(DOC_DIR) $(LIB_FILE)

# Examples

examples: example1

example1: lib
	mkdir -p $(EXAMPLE_DIR)
	$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(EXAMPLE_DIR) -O examples/example1.rs
	$(EXAMPLE_DIR)/example1
	cd $(EXAMPLE_DIR)
	convert example1.pgm example1.png
	rm example1.pgm
