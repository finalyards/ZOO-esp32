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

# tbd. apply the 'patch' to the provided ULD_API sources, once
#		- or... treat the source as read-only and make a local copy (of only the needed files) under 'tmp'? :) +1

#TARGET?=riscv32imac-unknown-none-elf
  # C3: riscv32imc-unknown-none-elf
  # C6: riscv32imac-unknown-none-elf

_ULD_PATH:=../VL53L5CX_ULD_API
_ULD_SRC:=$(_ULD_PATH)/src/vl53l5cx_api.c

_TMP:=./tmp/
#_SO:=$(_TMP)/$(TARGET)/uld.so

_OBJ_PATH:=$(_TMP)/$(TARGET)

_INCS:=$(_ULD_PATH)/inc/* platform.h fake/*.h

all: #${_OUTPUT}
	@false

#${_OUTPUT}: ./wrapper.h ${_API_INC_PATH}/vl53l5cx_api.h
#	cargo build --target ${TARGET}

#---
# Build the static library
#
# Note: We don't really _need_ a static library: it's just a single '.o' file, but in order to follow the normal
#		workflow, also 'ar' is used.
#
# Note 2: '--thin' means the archive simply points to the '.o' file, which is expected to remain available. This is
#		way over-optimization, but since I happened to read the whole 'man ar', why not.. :)
#		[[it would work without '--thin']]
#
tmp/_.a: tmp/vl53l5cx_api.o
	ar rcs --thin $@ $<

tmp/%.o: $(_ULD_PATH)/src/%.c $(_INCS)
	@echo $<
	clang -nostdinc --target=riscv32-unknown-elf -Ifake -I. -I$(_ULD_PATH)/inc -c -o $@ $<

#---
# Generate
#
# Notes:
#	- '--formatter=prettyplease' (not needing nightly?) didn't work
#	- using 'rustfmt.toml' > normalize_doc_attributes requires nightly

# 'platform.h' conversion is ever only used manually; the 'src/platform.rs' was based on it.
#
tmp/platform-raw.templ.rs: platform.h Makefile
	rustup run nightly \
	  bindgen $< --allowlist-function 'VL53L5CX_.+' \
	    --opaque-type=VL53L5CX_Platform \
	    --use-core \
	  | sed s/TimeMs/time_ms/ \
	  | sed s/RegisterAdress/register_address/ > $@

# 'vl53l5cx_api.h' conversion is ...[[tbd.]]
#
src/uld-raw.rs: $(_ULD_PATH)/inc/vl53l5cx_api.h Makefile
	rustup run nightly \
	  bindgen $< \
	  	--allowlist-var 'VL53L5CX_.+' --allowlist-type 'VL53L5CX_.+' \
		--allowlist-item 'VL53L5CX_.+' \
		--allowlist-function 'vl53l5cx_.+' \
		\
		--blocklist-type 'VL53L5CX_Platform' \
		--blocklist-function 'VL53L5CX_RdByte' \
		--blocklist-function 'VL53L5CX_WrByte' \
		--blocklist-function 'VL53L5CX_RdMulti' \
		--blocklist-function 'VL53L5CX_WrMulti' \
		--blocklist-function 'VL53L5CX_Reset_Sensor' \
		--blocklist-function 'VL53L5CX_SwapBuffer' \
		--blocklist-function 'VL53L5CX_WaitMs' \
	    --use-core \
	    --no-layout-tests \
	    -- -I. \
	  | sed s/VL53L5CX_Platform/Platform/ \
	  > $@
	@# tbd. enable layout tests once done!
	@# tbd. how to NOT have the things from 'platform.h'?  '--blocklist-file=platform.h' did not cut!

_clean:
	cargo clean

echo:
	@echo X

#--
.PHONY: all _clean echo
