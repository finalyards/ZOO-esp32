#
# THIS MAKEFILE IS ONLY FOR DEVELOPMENT - not needed for making production builds
#

TARGET=riscv32imc-unknown-none-elf
CHIP=esp32c3

all:
	@false

ellie:
	cargo build --bin ellie --features $(CHIP) --target $(TARGET)

echo:
	@echo X

#---
.PHONY: all ellie echo
