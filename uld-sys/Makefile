#
# Makefile for 'VL53L5CX_ULD_API' wrapper
#
# Usage:
#	<<
#	  $ [TARGET=...] make
#	<<
#
# Requires:
#	- bindgen
#
# Note: 'bindgen' docs show how that tool can also build the static library (as part of the 'cargo build' command run).
#		We take a conventional approach, building the static library here in the Makefile. Mostly a matter of taste.
#

TARGET?=riscv32imac-unknown-none-elf
  # C3: riscv32imc-unknown-none-elf
  # C6: riscv32imac-unknown-none-elf

_ULD_PATH:=../VL53L5CX_ULD_API
_ULD_SRC:=$(_ULD_PATH)/src/vl53l5cx_api.c

_TMP:=./tmp/
_SO:=$(_TMP)/$(TARGET)/uld.so

_OBJ_PATH:=$(_TMP)/$(TARGET)

_CLANG_FLAGS:=--target riscv32

all: #${_OUTPUT}
	@false

#${_OUTPUT}: ./wrapper.h ${_API_INC_PATH}/vl53l5cx_api.h
#	cargo build --target ${TARGET}

x: $(_SO)		#TEMP

# Build the static library
#
# Note: We don't need to optimize this piece-meal. If something in the sources has changed, we'll recompile and link
#		everything.
#
$(_SO): $(_ULD_PATH)/inc/* $(_ULD_PATH)/src/*
	@false	# unfinished - how to provide the target to the compiler???
	clang $(_CLANG_FLAGS) -c -o $(_OBJ_PATH) $(_ULD_SRC)
	#ar rcs $(_LIB_PATH) $(_OBJ_PATH)

# Generate
#
# Notes:
#	- '--formatter=prettyplease' (not needing nightly?) didn't work
#	- using 'rustfmt.toml' > normalize_doc_attributes requires nightly
#
src/platform-raw.templ.rs: platform.h Makefile
	rustup run nightly \
	  bindgen $< --allowlist-function 'VL53L5CX_.+' \
	  --opaque-type=VL53L5CX_Platform \
	  --use-core \
	  | sed s/TimeMs/time_ms/ \
	  | sed s/RegisterAdress/register_address/ > $@

_clean:
	cargo clean

echo:
	@echo X

#--
.PHONY: all _clean echo
