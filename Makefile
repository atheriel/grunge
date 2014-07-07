RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/noise.rs
TEST_FILE           = $(SRC_DIR)/test.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

DEPS_DIR            = target/deps
LIB_DIR             = target
TEST_DIR            = target/test
EXAMPLE_DIR         = target/examples
DOC_DIR             = doc

all: lib doc

lib: $(LIB_FILE)
	mkdir -p $(LIB_DIR)
	$(RUSTC) -L $(DEPS_DIR) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

test: lib
	mkdir -p $(TEST_DIR)
	$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(TEST_DIR) --test $(TEST_FILE)
	$(TEST_DIR)/test

examples: example1

example1: lib
	mkdir -p $(EXAMPLE_DIR)
	$(RUSTC) -L $(LIB_DIR) -L $(DEPS_DIR) --out-dir=$(EXAMPLE_DIR) examples/example1.rs
	$(EXAMPLE_DIR)/example1
	convert example1.pgm $(EXAMPLE_DIR)/example1.png
	rm example1.pgm

doc:
	mkdir -p $(DOC_DIR)
	$(RUSTDOC) --html-in-header src/doc/mathjax.html -o $(DOC_DIR) $(LIB_FILE)

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(TEST_DIR)
	rm -rf $(BENCH_DIR)
	rm -rf $(DOC_DIR)

.PHONY: \
	all \
	lib \
	test \
	bench \
	check \
	doc \
	clean