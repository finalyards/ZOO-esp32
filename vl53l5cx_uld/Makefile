#
# This Makefile is only for help - a wrapper around 'cargo build' to shorten the commands.
#
# Usage:
#	$ [FEATURES=...] make {nada|ranging_basic|...}
#		# builds and runs a certain sample
#
CHIP:=esp32c3

PROFILE?=release
DEFMT_LOG?=debug
FEATURES?=

LOG_ENV:=DEFMT_LOG=$(DEFMT_LOG)

DEFMT_HOST_FMT:='[{L:bold}] {s}'
DEFMT_FEATURES:=defmt,$(FEATURES)

ifeq (dev,$(PROFILE))
--PROFILE:=--profile dev
else ifeq (release,$(PROFILE))
--PROFILE:=--release
else
  $(error Unknown 'PROFILE': "$(PROFILE)")
endif

ifeq (esp32c3,$(CHIP))
TARGET:=riscv32imc-unknown-none-elf
else ifeq (esp32c6,$(CHIP))
TARGET:=riscv32imac-unknown-none-elf
else
  $(error Unexpected 'CHIP': "$(CHIP)")
endif

# List names of 'examples/[0-9]*.rs'
EXAMPLE_NAMES:=$(notdir $(basename $(wildcard examples/_[0-9]*.rs) $(wildcard examples/[0-9]*.rs)))
	# '_' prefix used until 'defmt' allows non-identifier file names

all:
	@false

# Note: Though trying in multiple ways, did not find a way to make 'cargo run --example' itself apply the 'defmt'
#		feature.

#TEMP
nada:
	$(LOG_ENV) cargo build $(--PROFILE) --features=defmt --example $@
	probe-rs run --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/$(PROFILE)/examples/nada

nada!:
	probe-rs run --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/$(PROFILE)/examples/nada

1 ranging-basic:
	FEATURES=distance_mm $(MAKE) _1-ranging_basic

2 get-set-parameters:
	FEATURES=distance_mm $(MAKE) _2-get_set_parameters

$(EXAMPLE_NAMES):
	$(LOG_ENV) cargo build --timings $(--PROFILE) --features=$(DEFMT_FEATURES) --example $@
	probe-rs run --speed=200 --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/$(PROFILE)/examples/$@

echo:
	@echo $(EXAMPLE_NAMES)

.PHONY: all lib $(EXAMPLE_NAMES) _clean echo
