#
# THIS MAKEFILE IS ONLY FOR DEVELOPMENT - not needed for making production builds
#

TARGET:=riscv32imc-unknown-none-elf
CHIP:=esp32c3

_ELLIE_DEBUG_TARGET:=target/${TARGET}/debug/ellie

_DEFMT_LOG=debug

all:
	@false

# Simple target with no connection to 'uld'.  Test workflow (all the way to flashing).
ellie:
	DEFMT_LOG=${_DEFMT_LOG} \
	  cargo build --bin ellie --features $(CHIP) --target $(TARGET)

#ellie-run:
#	cargo run --bin ellie --features $(CHIP) --target $(TARGET)

ellie-run: $(_ELLIE_DEBUG_TARGET)
	$(MAKE) ellie-run!

ellie-run!: | $(_ELLIE_DEBUG_TARGET)
	espflash flash --monitor $|

$(_ELLIE_DEBUG_TARGET): src/bin/ellie.rs ./Cargo.toml
	@{MAKE} ellie
	@test -f $@

clean:
	-rm $(_ELLIE_DEBUG_TARGET)

echo:
	@echo X

#---
.PHONY: all ellie echo
