CARGO ?= cargo

AARCH64_TARGET := aarch64-unknown-linux-gnu
AARCH64_PREFIX ?= aarch64-linux-gnu-
AARCH64_LINKER ?= $(AARCH64_PREFIX)gcc

JIMIRU_LIB_FILES != find jimiru -type f
JIMIRU_SERVER_FILES := $(JIMIRU_LIB_FILES) $(shell find jimiru_server -type f)
JIMIRU_JITAKU_FILES := $(JIMIRU_LIB_FILES) $(shell find jimiru_jitaku -type f)

# TODO: うまい判定方法〜〜
# OPENSSL_FILES != find external/openssl -type f -and \( -name '*.c' -or -name '*.h' -or -name '*.pl' -or -name '*.S' \)

OPENSSL_DEST := external/openssl/dest/libcrypto.a external/openssl/dest/libcrypto.so external/openssl/dest/libssl.a external/openssl/dest/libssl.so
OPENSSL_DEST_AARCH64 := external/openssl/dest_aarch64/libcrypto.a external/openssl/dest_aarch64/libcrypto.so external/openssl/dest_aarch64/libssl.a external/openssl/dest_aarch64/libssl.so

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

openssl:
	cd external/openssl; \
	./Configure; \
	$(MAKE); \
	mkdir -p dest; \
	cp libcrypto.a libcrypto.so libssl.a libssl.so dest/

external/openssl/dest/libcrypto.a:
	$(MAKE) openssl

external/openssl/dest/libcrypto.so:
	$(MAKE) openssl

external/openssl/dest/libssl.a:
	$(MAKE) openssl

external/openssl/dest/libssl.so:
	$(MAKE) openssl

openssl_aarch64:
	cd external/openssl; \
	./Configure --cross-compile-prefix=$(AARCH64_PREFIX) linux-aarch64; \
	$(MAKE); \
	mkdir -p dest_aarch64; \
	cp libcrypto.a libcrypto.so libssl.a libssl.so dest_aarch64/

external/openssl/dest_aarch64/libcrypto.a:
	$(MAKE) openssl_aarch64

external/openssl/dest_aarch64/libcrypto.so:
	$(MAKE) openssl_aarch64

external/openssl/dest_aarch64/libssl.a:
	$(MAKE) openssl_aarch64

external/openssl/dest_aarch64/libssl.so:
	$(MAKE) openssl_aarch64

clean:
	rm -rf target

.PHONY: all_release all_debug release aarch64_release debug aarch64_debug openssl openssl_aarch64 clean

# OpenSSL のコンパイルが並列実行不可能
# 他もどうせ cargo 叩くだけだしいいでしょ
.NOTPARALLEL:
