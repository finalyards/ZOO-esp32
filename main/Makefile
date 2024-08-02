#
# THIS MAKEFILE IS ONLY FOR DEVELOPMENT - not needed for making production builds
#
CHIP?=esp32c6
PROFILE?=release
LOG_LEVEL:=debug

# would like to use 'defmt' always (for C3 as well..)

ifneq (,$(findstring $(CHIP),esp32c3 ..))
  TARGET:=riscv32imc-unknown-none-elf
else ifneq (,$(findstring $(CHIP),esp32c6 ..))
  TARGET:=riscv32imac-unknown-none-elf
  DEFMT:=1
else
  $(error Unknown 'CHIP': "$(CHIP)")
endif

ifdef $(DEFMT)
  DEFMT_FEATURE:=defmt
else
  DEFMT_FEATURE:=non_defmt
endif

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

null  :=
space := $(null) #
comma := ,

_FEATURES_CSL := $(subst $(space),$(comma),$(strip $(CHIP) $(DEFMT_FEATURE)))
	# comma separated

all:
	@false

# Simple target with no connection to 'uld'. Test workflow (all the way to flashing and monitoring).
ellie:
	DEFMT_LOG=$(LOG_LEVEL) cargo build $(--PROFILE) --bin $(_APP) --features=$(_FEATURES_CSL) --target=$(TARGET)

# Nightly would have some benefits (tbd. list them; see if it's worth)
#ellie-with-nightly:
#	cargo +nightly-2024-06-01 build $(--PROFILE) --bin $(_APP) --features=nightly,$(_FEATURES_CSL) --target=$(TARGET)

$(_APP_BIN): src/bin/*.rs Makefile Cargo.toml .cargo/config.toml ./build.rs
	@$(MAKE) CHIP=$(CHIP) $(_APP)
	@test -f $@

ellie-run: $(_APP_BIN)
ifeq ($(CHIP),esp32c3)
	$(warn 'probe-rs run' is glitchy for '$(CHIP)')
endif
	probe-rs run --chip $(CHIP) $<

ellie-run2: $(_APP_BIN)
	espflash flash --monitor $<

_clean:
	-cargo clean

echo:
	@echo X

#---
.PHONY: all ellie ellie-run ellie-run2 _clean echo
