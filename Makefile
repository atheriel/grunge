SHELL               = bash

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
DOC_TEST_PARAMS     = -L $(LIB_DIR) --test

.PHONY: all lib test bench check doc clean help

all: lib doc

clean:
	@echo "--- Removing generated files:"
	rm -rf $(LIB_DIR)
	rm -rf $(DOC_DIR)

help:
	@echo "--- Available Options:"
	@echo "make             - Build the library & documentation."
	@echo "make lib         - Build the library."
	@echo "make test        - Run the unit tests."
	@echo "make bench       - Run benchmarks."
	@echo "make doc         - Builds the library's documentation."
	@echo "make doctest     - Runs the examples in the documentation."
	@echo "make examples    - Builds the examples."
	@echo "make clean       - Removes all generated files."

# Library

lib: $(LIB_FILE)
	@echo "--- Building library."
	@mkdir -p $(LIB_DIR)
	@rm -f $(LIB_DIR)/libgrunge*.rlib  # Quick fix for using Cargo + make together
	@$(RUSTC) -L $(DEPS_DIR) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

# Testing and Benchmarking

test: lib
	@echo "--- Building tests."
	@mkdir -p $(TEST_DIR)
	@$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(TEST_DIR) --test $(TEST_FILE)
	@echo "--- Running tests:"
	@$(TEST_DIR)/test

bench: test
	@echo "--- Running benchmarks:"
	$(TEST_DIR)/test --bench

# Documentation

doc:
	@echo "--- Generating documentation."
	@mkdir -p $(DOC_DIR)
	@$(RUSTDOC) $(DOC_PARAMS) -o $(DOC_DIR) $(LIB_FILE)

doctest:
	@echo "--- Running documentation examples:"
	@mkdir -p $(DOC_DIR)
	@$(RUSTDOC) $(DOC_PARAMS) $(DOC_TEST_PARAMS) -o $(DOC_DIR) $(LIB_FILE)

# Examples

examples: example1 example2

example1: lib
	@echo "--- Building example #1."
	@mkdir -p $(EXAMPLE_DIR)
	@$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(EXAMPLE_DIR) -O examples/example1.rs
	@echo "--- Running example #1:"
	@$(EXAMPLE_DIR)/example1

example2: lib
	@echo "--- Building example #2."
	@mkdir -p $(EXAMPLE_DIR)
	@$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(EXAMPLE_DIR) -O examples/example2.rs
	@echo "--- Running example #2:"
	@$(EXAMPLE_DIR)/example2
