#
# THIS MAKEFILE IS ONLY FOR DEVELOPMENT - not needed for making production builds
#
CHIP?=esp32c6
PROFILE?=release
DEFMT_LOG:=debug

ifneq (,$(findstring $(CHIP),esp32c3 ..))
  TARGET:=riscv32imc-unknown-none-elf
else ifneq (,$(findstring $(CHIP),esp32c6 ..))
  TARGET:=riscv32imac-unknown-none-elf
else
  $(error Unknown 'CHIP': "$(CHIP)")
endif

LOG_ENV:=DEFMT_LOG=$(DEFMT_LOG)

_APP:=ellie

ifeq (dev,$(PROFILE))
_APP_BIN:=target/$(TARGET)/debug/$(_APP)
--PROFILE:=--profile dev
else ifeq (release,$(PROFILE))
_APP_BIN:=target/$(TARGET)/release/$(_APP)
--PROFILE:=--release
else
  $(error Unknown 'PROFILE': "$(PROFILE)")
endif

_FEATURES_CSL:=$(CHIP)
 	# comma separated

all:
	@false

# Simple target with no connection to 'uld'. Test workflow (all the way to flashing and monitoring).
ellie:
	$(LOG_ENV) cargo build $(--PROFILE) --bin $(_APP) --features=$(_FEATURES_CSL) --target=$(TARGET)

# Nightly would have some benefits (tbd. list them; see if it's worth)
#ellie-with-nightly:
#	cargo +nightly-2024-06-01 build $(--PROFILE) --bin $(_APP) --features=nightly,$(_FEATURES_CSL) --target=$(TARGET)

$(_APP_BIN): src/bin/*.rs Makefile Cargo.toml .cargo/config.toml ./build.rs
	@$(MAKE) CHIP=$(CHIP) $(_APP)
	@test -f $@

ellie-run: $(_APP_BIN)
	probe-rs run --chip $(CHIP) --log-format '{L} {s}' $<
		# for log format, see -> https://defmt.ferrous-systems.com/custom-log-output

# Run without recompiling (even checking for) the binary; faster
ellie-run!:
	probe-rs run --chip $(CHIP) --log-format '{L} {s}' $(_APP_BIN)

_clean:
	-cargo clean

echo:
	@echo X

#---
.PHONY: all ellie ellie-run ellie-run! _clean echo
