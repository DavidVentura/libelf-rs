FUZZ_CPUS := 4
FUZZ_SECS := 60

PROJECT_ROOT := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
LIBELF_RS_ROOT := $(shell dirname $(PROJECT_ROOT))
LIBELF_BUILD_DIR := $(LIBELF_RS_ROOT)/target/release
INCLUDE_DIR := $(LIBELF_RS_ROOT)/include

ifndef LIBBPF_CHECKOUT_DIR
$(error LIBBPF_CHECKOUT_DIR is not set. Usage: make fuzz_libbpf LIBBPF_CHECKOUT_DIR=/path/to/libbpf)
endif

LIBBPF_SRC := $(LIBBPF_CHECKOUT_DIR)/src
LIBBPF_FUZZ := $(LIBBPF_CHECKOUT_DIR)/fuzz

ifeq (,$(wildcard $(LIBBPF_SRC)))
$(error libbpf src directory not found at $(LIBBPF_SRC))
endif

FUZZING_FLAGS := -O1 -fno-omit-frame-pointer -g -DFUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION -fsanitize=address -fsanitize=fuzzer-no-link
COVERAGE_FLAGS := -fprofile-instr-generate -fcoverage-mapping
CC := clang
CXX := clang++
LIB_FUZZING_ENGINE := -fsanitize=fuzzer
OUT := $(LIBBPF_CHECKOUT_DIR)/out

LIBBPF_CFLAGS := -I$(INCLUDE_DIR) $(FUZZING_FLAGS)
LIBBPF_LDFLAGS := -L$(LIBELF_BUILD_DIR) -Wl,-rpath,$(LIBELF_BUILD_DIR)

.PHONY: build_libelf build_libbpf fuzz_libbpf clean_libbpf

build_libelf:
	cd $(LIBELF_RS_ROOT) && \
	RUSTFLAGS="-C instrument-coverage" cargo build --release

build_libbpf: build_libelf
	$(MAKE) -C $(LIBBPF_SRC) clean
	$(MAKE) -C $(LIBBPF_SRC) \
		CC=$(CC) \
		CFLAGS="-I$(INCLUDE_DIR) $(FUZZING_FLAGS)" \
		BUILD_STATIC_ONLY=y \
		V=1

clean_libbpf:
	$(MAKE) -C $(LIBBPF_SRC) clean
	rm -rf $(OUT)

fuzz_libbpf: build_libbpf
	mkdir -p $(OUT)
	$(CC) $(FUZZING_FLAGS) $(COVERAGE_FLAGS) \
		-I$(LIBBPF_CHECKOUT_DIR)/src \
		-I$(LIBBPF_CHECKOUT_DIR)/include \
		-I$(LIBBPF_CHECKOUT_DIR)/include/uapi \
		-D_LARGEFILE64_SOURCE -D_FILE_OFFSET_BITS=64 \
		-c $(LIBBPF_CHECKOUT_DIR)/fuzz/bpf-object-fuzzer.c \
		-o $(OUT)/bpf-object-fuzzer.o
	$(CXX) $(FUZZING_FLAGS) $(COVERAGE_FLAGS) $(LIB_FUZZING_ENGINE) \
		$(OUT)/bpf-object-fuzzer.o \
		$(LIBBPF_SRC)/libbpf.a \
		$(LIBELF_BUILD_DIR)/liblibelf_rs.a \
		-l:libz.a \
		-o $(OUT)/bpf-object-fuzzer
	unzip -q -o $(LIBBPF_CHECKOUT_DIR)/fuzz/bpf-object-fuzzer_seed_corpus.zip -d /tmp/fuzz_corpus
	mkdir -p $(OUT)/profraw
	for testcase in /tmp/fuzz_corpus/*.o; do \
		echo "  $$testcase"; \
		LLVM_PROFILE_FILE="$(OUT)/profraw/coverage-%p.profraw" \
			$(OUT)/bpf-object-fuzzer $$testcase || exit 1; \
	done
	LLVM_PROFILE_FILE="$(OUT)/profraw/coverage-%p.profraw" \
		$(OUT)/bpf-object-fuzzer /tmp/fuzz_corpus \
		-dict=$(PROJECT_ROOT)/elf.dict \
		-use_value_profile=1 \
		-fork=$(FUZZ_CPUS) \
		-max_total_time=$(FUZZ_SECS) \
		-max_len=16384
	llvm-profdata merge -sparse $(OUT)/profraw/*.profraw -o $(OUT)/coverage.profdata
	llvm-cov show $(OUT)/bpf-object-fuzzer \
		-instr-profile=$(OUT)/coverage.profdata \
		-format=html -output-dir=$(OUT)/coverage-html \
		-show-line-counts-or-regions \
		-show-instantiations=false \
		$(LIBELF_RS_ROOT)/src
	llvm-cov report $(OUT)/bpf-object-fuzzer \
		-instr-profile=$(OUT)/coverage.profdata \
		$(LIBELF_RS_ROOT)/src
