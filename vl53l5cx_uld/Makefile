#
# Makefile for ULD (= C driver) wrapper
#
# Gets called by 'cargo build' (but can be used manually as well!).
#
# Usage:
#	<<
#	  $ make [tmp/vendor_uld.a] [src/uld_raw.rs]
#	<<
#
# Requires:
#	- bindgen
#	- patch
#
# Note: 'bindgen' docs show how that tool can also build the static library as part of the 'cargo build' command run.
#		We kind of follow that approach, but wrap all the C side actions still under a 'Makefile'.
#

#---
# NOTE: 'clang' only recognizes the 'riscv32-unknown-elf' target, not the more fine grained ones.
#	<<
#	  $ clang -print-targets | grep risc
#     riscv32     - 32-bit RISC-V
#	  riscv64     - 64-bit RISC-V
#	<<
#
#	Related:
#		- "RISC-V has different triples for clang and rustc" (2021; closed)
#			-> https://github.com/rust-lang/rust-bindgen/issues/2136
#		- "RISC-V target not passed on correctly to clang" (2019; open)
#			-> https://github.com/rust-lang/rust-bindgen/issues/1555
#
_VL53L5CX_ULD_API=VL53L5CX_ULD_API

_TMP:=./tmp/
_C_SRC:=tmp/c_src

_OBJ_PATH:=$(_TMP)/$(TARGET)

_OTHER_INCS:=platform.h fake/*.h

_TARGET_ARCH:=riscv32-unknown-elf

all:
	@false

#---
# Build the static library
#
# Note: '--thin' means the archive simply points to the '.o' file, which is expected to remain available. This is
#		way over-optimization, but since I happened to read the whole 'man ar', why not.. :) [[it would work without '--thin']]
#
tmp/vendor_uld.a: tmp/wrap.o tmp/vl53l5cx_api.o
	ar rcs --thin $@ $<

# Note: Dependency on features is via 'Cargo.toml'. 'build.rs' rebuilds 'tmp/config.h' on each run, so it's not
#		a good idea to use it (would cause everything to recompile, all the time). This may be overthinking, but
#		..it works.
#
tmp/%.o: $(_C_SRC)/%.c wrap.h $(_C_SRC)/*.h $(_OTHER_INCS) Cargo.toml
	@echo $<
	clang -nostdinc --target=$(_TARGET_ARCH) -Ifake -I. -I$(_C_SRC) -c -o $@ $<

tmp/wrap.o: wrap.c wrap.h $(_C_SRC)/*.h $(_OTHER_INCS) Cargo.toml
	@echo $<
	clang -nostdinc --target=$(_TARGET_ARCH) -Ifake -I. -I$(_C_SRC) -c -o $@ $<

#---
# Patch ST.com sources from './VL53LCX_ULD_API/**' to 'tmp/c_src/'
#
# 	- places e.g. headers and C files in the same folder, for convenience
#	- removes a nasty! reference to a field in a structure that's supposed to be customer-made and opaque
#
$(_C_SRC)/%.h: $(_VL53L5CX_ULD_API)/inc/%.h |_c_src_mkdir
	@cp $< $@

$(_C_SRC)/vl53l5cx_api.c: $(_VL53L5CX_ULD_API)/src/vl53l5cx_api.c ./patch |_c_src_mkdir
	patch $< ./patch -o $@

# Error at Make root level, if vendor sources aren't there
ifeq ("$(wildcard $(_VL53L5CX_ULD_API)/src/*)", "")
$(error ST.com ULD driver not found; please download and place in the folder $(_VL53L5CX_ULD_API))
endif

_c_src_mkdir:
	@install -d $(_C_SRC)

# Make sure features have been processed.
# We don't use regular 'tmp/config.h:' target, because it would not work with 'make -B' (force rebuilds).
ifeq ("$(wildcard tmp/config.h)", "")
$(error No 'tmp/config.h'. Either create an empty file or run once with 'cargo build')
endif

#---
# Generate Rust
#
# Notes:		TBD: IS THE COMMENT OUTDATED ---v ?
#	- '--formatter=prettyplease' (not needing nightly?) didn't work
#	- using 'rustfmt.toml' > normalize_doc_attributes requires nightly

# 'platform.h' conversion was ever only used manually; the 'src/platform.rs' is based on it.
#
tmp/platform-raw.templ.rs: platform.h Makefile
	rustup run stable \
	  bindgen $< --allowlist-function 'VL53L5CX_.+' \
	    --opaque-type=VL53L5CX_Platform \
	    --use-core \
	  | sed s/TimeMs/time_ms/ \
	  | sed s/RegisterAdress/register_address/ > $@

# 'vl53l5cx_api.h' conversion is automatic (changes to 'src/uld_raw.rs' WILL BE LOST; modify this instead)
#
# About 'bindgen' (v.0.69.4):
#	- prints errors to stdout, which is .... a bit unkind.
#	- does not process lines like:
#		<<
#			define VL53L5CX_RESOLUTION_4X4			((uint8_t) 16U)
#		<<
#	  If, instead, we turn those to:
#			const uint8_t VL53L5CX_RESOLUTION_4X4 = 16;
#	  ..there is no problem.
#
# Note:
#	The author didn't get '--blocklist-file' to work. It didn't deny the functions in 'platform.h' - but that's just 8 names.
#
src/uld_raw.rs: wrap-wrap.h wrap.h $(_C_SRC)/vl53l5cx_api.h $(_C_SRC)/vl53l5cx_buffers.h $(_OTHER_INCS) Makefile Cargo.toml
	rustup run stable \
	  bindgen $< \
		--allowlist-file $< \
		--allowlist-file 'wrap.h' \
	    --use-core \
	    --default-enum-style rust \
	    --raw-line '#![no_std]' \
	    --raw-line '#![allow(non_camel_case_types)]' \
	    --raw-line '#![allow(non_snake_case)]' \
	    --must-use-type STATUS \
	    -- -I. -I$(_C_SRC) \
	  | sed s/VL53L5CX_Platform/Platform/ \
	  > $@

#R		--blocklist-function 'VL53L5CX_?(RdByte|WrByte|RdMulti|WrMulti|Reset_Sensor|SwapBuffer|WaitMs)' \

# Test for the above (for development, only)
x: src/uld_raw.rs
	grep -q DEFAULT_I2C_ADDRESS $<
	grep -q 'pub enum POWER_MODE' $<
	grep -q 'static VL53L5CX_FIRMWARE' $<
	grep -vq VL53L5CX_RdByte $<
	@#
	@echo ""
	@echo "Yay!"

_clean:
	cargo clean

echo:
	@echo X

#--
.PHONY: all _c_src_mkdir _clean echo
