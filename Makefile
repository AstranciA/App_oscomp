AX_MAKE_DEFAULTS ?= BLK=y FEATURES=lwext4_rs,fp_simd,fs, DISK_IMG=disk2.img

TOOLCHAIN_DIR ?= ../toolchains
AX_SOURCE ?= git:https://github.com/AstranciA/AstrancE.git
AX_ROOT ?= .AstrancE
T_ROOT ?= $(PWD)/testcases
T ?= nimbos
ARCH ?= x86_64
LOG ?= off
#AX_TESTCASES_LIST=$(shell cat ./apps/$(AX_TESTCASE)/testcase_list | tr '\n' ',')
FEATURES ?= fp_simd

TESTCASE := $(T_ROOT)/$(T)

RUSTDOCFLAGS := -Z unstable-options --enable-index-page -D rustdoc::broken_intra_doc_links -D missing-docs
EXTRA_CONFIG ?= $(PWD)/configs/$(ARCH).toml
ifneq ($(filter $(MAKECMDGOALS),doc_check_missing),) # make doc_check_missing
    export RUSTDOCFLAGS
else ifeq ($(filter $(MAKECMDGOALS),clean user_apps ax_root),) # Not make clean, user_apps, ax_root
    export AX_TESTCASES_LIST
endif

DIR := $(shell basename $(PWD))
OUT_ELF := $(DIR)_$(ARCH)-qemu-virt.elf
OUT_BIN := $(DIR)_$(ARCH)-qemu-virt.bin

# Target
ifeq ($(ARCH), x86_64)
  TARGET := x86_64-unknown-none
else ifeq ($(ARCH), aarch64)
  ifeq ($(findstring fp_simd,$(FEATURES)),)
    TARGET := aarch64-unknown-none-softfloat
  else
    TARGET := aarch64-unknown-none
  endif
else ifeq ($(ARCH), riscv64)
  TARGET := riscv64gc-unknown-none-elf
else ifeq ($(ARCH), loongarch64)
  TARGET := loongarch64-unknown-none
else
  $(error ARCH must be one of x86_64, aarch64, riscv64, loongarch64)
endif

include scripts/make/oscomp.mk

all: env oscomp_build

env:
	export PATH=$(TOOLCHAIN_DIR)/bin:$(PATH)

# export dummy config for clippy
clippy: defconfig
	@AX_CONFIG_PATH=$(PWD)/.axconfig.toml cargo clippy --target $(TARGET) --all-features -- -D warnings -A clippy::new_without_default	

fetch_ax:
	@./scripts/fetch.sh $(AX_SOURCE) $(AX_ROOT)

ax_root:
	@./scripts/set_ax_root.sh $(AX_ROOT)
	@make -C $(AX_ROOT) disk_img

testcase:
	#@make -C $(TESTCASE) ARCH=$(ARCH) build
	@make -C $(TESTCASE) ARCH=$(ARCH) rust
	@if [ -z "$(shell command -v sudo)" ]; then \
		./build_img.sh -a $(ARCH) -fs ext4 -file $(TESTCASE)/build/$(ARCH) -s 20; \
	else \
		sudo ./build_img.sh -a $(ARCH) -fs ext4 -file $(TESTCASE)/build/$(ARCH) -s 20; \
	fi
	@mv ./disk.img $(AX_ROOT)/disk.img

test: defconfig
	@./scripts/app_test.sh

defconfig build run justrun debug disasm:env ax_root
	@make -C $(AX_ROOT) A=$(PWD) EXTRA_CONFIG=$(EXTRA_CONFIG) $(AX_MAKE_DEFAULTS) $@

clean: ax_root
	@make -C $(AX_ROOT) A=$(PWD) ARCH=$(ARCH) clean
	@for dir in $(shell ls ./apps); do \
		make -C ./apps/$$dir clean; \
	done
	@cargo clean

doc_check_missing:
	@cargo doc --no-deps --all-features --workspace

.PHONY: all ax_root build run justrun debug disasm clean test_build

