#
# This Makefile is only for help - a wrapper around 'cargo build' to shorten the commands.
#
# Usage:
#	$ [DEFMT_LOG=trace|debug|info|...] [FEATURES=...] make {nada|ranging_basic|...}
#		# builds and runs a certain sample
#
DEFMT_LOG?=debug
FEATURES?=

LOG_ENV:=DEFMT_LOG=$(DEFMT_LOG)

DEFMT_HOST_FMT:='[{L:bold}] {s}'
DEFMT_FEATURES:=defmt,$(FEATURES)

# Read the 'TARGET' from './cargo/config.toml'
#
TARGET:=$(shell cat .cargo/config.toml | grep -e '^target\s*=\s"' | cut -d '"' -f2)
	# riscv32imac-unknown-none-elf

# List names of 'examples/[0-9]*.rs'
EXAMPLE_NAMES:=$(notdir $(basename $(wildcard examples/_[0-9]*.rs) $(wildcard examples/[0-9]*.rs)))
	# '_' prefix used until 'defmt' allows non-identifier file names

all:
	@false

# Note: Though trying in multiple ways, did not find a way to make 'cargo run --example' itself apply the 'defmt'
#		feature.

#TEMP
nada:
	$(LOG_ENV) cargo build --release --features=defmt --example $@
	probe-rs run --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/release/examples/nada

nada!:
	probe-rs run --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/release/examples/nada

1 ranging-basic:
	FEATURES=distance_mm $(MAKE) _1-ranging_basic

2 get-set-parameters:
	FEATURES=target_status,distance_mm $(MAKE) _2-get_set_parameters

$(EXAMPLE_NAMES):
	$(LOG_ENV) cargo build --timings --release --features=$(DEFMT_FEATURES) --example $@
	probe-rs run --speed=200 --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/release/examples/$@

echo:
	@echo $(TARGET)

.PHONY: all lib $(EXAMPLE_NAMES) _clean echo
