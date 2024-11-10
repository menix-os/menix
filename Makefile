# Makefile for the kernel and modules

BUILD_DIR?=build
INSTALL_DIR?=build/install
JOBS?=$(shell nproc)
ARCH?=x86_64

.PHONY: all
all:
	cargo build --target toolchain/$(ARCH).json
