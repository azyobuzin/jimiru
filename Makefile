CARGO ?= cargo
JIMIRU_SERVER_FILES := $(shell find jimiru_server -type f)
JIMIRU_JITAKU_FILES := $(shell find jimiru_jitaku -type f)
AARCH64_TARGET := aarch64-unknown-linux-gnu
AARCH64_LINKER := aarch64-linux-gnu-gcc

.PHONY: all_release all_debug release arm_release debug arm_debug clean

all_release: release aarch64_release
all_debug: debug aarch64_debug

release: target/release/jimiru_server target/release/jimiru_jitaku
aarch64_release: target/$(AARCH64_TARGET)/release/jimiru_jitaku
debug: target/debug/jimiru_server target/debug/jimiru_jitaku
aarch64_debug: target/$(AARCH64_TARGET)/debug/jimiru_jitaku

target/release/jimiru_server: $(JIMIRU_SERVER_FILES)
	$(CARGO) build -p jimiru_server --release

target/release/jimiru_jitaku: $(JIMIRU_JITAKU_FILES)
	$(CARGO) build -p jimiru_jitaku --release

target/$(AARCH64_TARGET)/release/jimiru_jitaku: $(JIMIRU_JITAKU_FILES)
	$(CARGO) rustc -p jimiru_jitaku --release --target $(AARCH64_TARGET) -- -C linker=$(AARCH64_LINKER)

target/debug/jimiru_server: $(JIMIRU_SERVER_FILES)
	$(CARGO) build -p jimiru_server

target/debug/jimiru_jitaku: $(JIMIRU_JITAKU_FILES)
	$(CARGO) build -p jimiru_jitaku

target/$(AARCH64_TARGET)/debug/jimiru_jitaku: $(JIMIRU_JITAKU_FILES)
	$(CARGO) rustc -p jimiru_jitaku --target $(AARCH64_TARGET) -- -C linker=$(AARCH64_LINKER)

clean:
	rm -rf target
