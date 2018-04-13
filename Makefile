CARGO ?= cargo

AARCH64_TARGET := aarch64-unknown-linux-gnu
AARCH64_PREFIX ?= aarch64-linux-gnu-
AARCH64_LINKER ?= $(AARCH64_PREFIX)gcc

JIMIRU_LIB_SOURCE != find jimiru -type f
JIMIRU_SERVER_SOURCE := $(JIMIRU_LIB_SOURCE) $(shell find jimiru_server -type f)
JIMIRU_JITAKU_SOURCE := $(JIMIRU_LIB_SOURCE) $(shell find jimiru_jitaku -type f)

OPENSSL_STATIC ?= 1
ifneq (0, $(OPENSSL_STATIC))
	OPENSSL_LIBS := libcrypto.a libssl.a
else
	OPENSSL_LIBS := libcrypto.so libssl.so
endif

OPENSSL_SOURCE_DIR ?= external/openssl
OPENSSL_SOURCE := $(addprefix $(OPENSSL_SOURCE_DIR)/, $(shell cd $(OPENSSL_SOURCE_DIR) && git ls-files))
OPENSSL_DEST_DIR := dest
OPENSSL_DEST_FILES := $(addprefix $(OPENSSL_SOURCE_DIR)/$(OPENSSL_DEST_DIR)/, $(OPENSSL_LIBS))
OPENSSL_DEST_DIR_AARCH64 := dest_aarch64
OPENSSL_DEST_FILES_AARCH64 := $(addprefix $(OPENSSL_SOURCE_DIR)/$(OPENSSL_DEST_DIR_AARCH64)/, $(OPENSSL_LIBS))

CARGO_ENV := OPENSSL_LIB_DIR=$(abspath $(OPENSSL_SOURCE_DIR)/$(OPENSSL_DEST_DIR)) OPENSSL_INCLUDE_DIR=$(abspath $(OPENSSL_SOURCE_DIR)/include) OPENSSL_STATIC=$(OPENSSL_STATIC)
CARGO_ENV_AARCH64 := OPENSSL_LIB_DIR=$(abspath $(OPENSSL_SOURCE_DIR)/$(OPENSSL_DEST_DIR_AARCH64)) OPENSSL_INCLUDE_DIR=$(abspath $(OPENSSL_SOURCE_DIR)/include) OPENSSL_STATIC=$(OPENSSL_STATIC)

all_release: release aarch64_release
all_debug: debug aarch64_debug

release: target/release/jimiru_server target/release/jimiru_jitaku
aarch64_release: target/$(AARCH64_TARGET)/release/jimiru_jitaku
debug: target/debug/jimiru_server target/debug/jimiru_jitaku
aarch64_debug: target/$(AARCH64_TARGET)/debug/jimiru_jitaku

check: $(OPENSSL_DEST_FILES)
	$(CARGO_ENV) $(CARGO) check --all

target/release/jimiru_server target/release/jimiru_jitaku: $(JIMIRU_SERVER_SOURCE) $(OPENSSL_DEST_FILES)
	$(CARGO_ENV) $(CARGO) build -p $(notdir $@) --release

target/$(AARCH64_TARGET)/release/jimiru_jitaku: $(JIMIRU_JITAKU_SOURCE) $(OPENSSL_DEST_FILES_AARCH64)
	$(CARGO_ENV_AARCH64) $(CARGO) rustc -p jimiru_jitaku --release --target $(AARCH64_TARGET) -- -C linker=$(AARCH64_LINKER)

target/debug/jimiru_server target/debug/jimiru_jitaku: $(JIMIRU_SERVER_SOURCE) $(OPENSSL_DEST_FILES)
	$(CARGO_ENV) $(CARGO) build -p $(notdir $@)

target/$(AARCH64_TARGET)/debug/jimiru_jitaku: $(JIMIRU_JITAKU_SOURCE) $(OPENSSL_DEST_FILES_AARCH64)
	$(CARGO_ENV_AARCH64) $(CARGO) rustc -p jimiru_jitaku --target $(AARCH64_TARGET) -- -C linker=$(AARCH64_LINKER)

$(OPENSSL_DEST_FILES): $(OPENSSL_SOURCE)
	$(MAKE) clean_openssl
	cd $(OPENSSL_SOURCE_DIR) && \
	./config --prefix=/usr --openssldir=/etc/ssl && \
	$(MAKE) build_libs && \
	mkdir -p $(OPENSSL_DEST_DIR) && \
	cp $(OPENSSL_LIBS) $(OPENSSL_DEST_DIR)/

$(OPENSSL_DEST_FILES_AARCH64): $(OPENSSL_SOURCE)
	$(MAKE) clean_openssl
	cd $(OPENSSL_SOURCE_DIR) && \
	./Configure --prefix=/usr --openssldir=/etc/ssl --cross-compile-prefix=$(AARCH64_PREFIX) linux-aarch64 && \
	$(MAKE) build_libs && \
	mkdir -p $(OPENSSL_DEST_DIR_AARCH64) && \
	cp $(OPENSSL_LIBS) $(OPENSSL_DEST_DIR_AARCH64)/

clean: clean_openssl
	rm -rf \
		target \
		$(OPENSSL_SOURCE_DIR)/$(OPENSSL_DEST_DIR) \
		$(OPENSSL_SOURCE_DIR)/$(OPENSSL_DEST_DIR_AARCH64)

clean_openssl:
	cd $(OPENSSL_SOURCE_DIR); \
	if [ -s Makefile ]; then \
		$(MAKE) clean; \
	fi

.PHONY: all_release all_debug release aarch64_release debug aarch64_debug check openssl openssl_aarch64 clean clean_openssl

# OpenSSL のコンパイルが並列実行不可能
# 他もどうせ cargo 叩くだけだしいいでしょ
.NOTPARALLEL:
