export ARCH ?= x86_64

CARGO_FLAGS=-Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

.PHONY: all
all:
	cargo build --target kernel/arch/$(ARCH)/$(ARCH).json $(CARGO_FLAGS)

