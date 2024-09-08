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

DEFMT_HOST_FMT:='{t:dimmed} [{L:bold}] {s}'
DEFMT_FEATURES:=defmt

# Read the 'TARGET' from './cargo/config.toml'
#
TARGET:=$(shell cat .cargo/config.toml | grep -e '^target\s*=\s"' | cut -d '"' -f2)
	# riscv32imac-unknown-none-elf

all:
	@false

# Note: Though trying in multiple ways, did not find a way to make 'cargo run --example' itself apply the 'defmt'
#		feature.

_nada:
	$(LOG_ENV) cargo build --release --features=defmt --example $@
	probe-rs run --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/release/examples/nada

m matrix:
	EXAMPLE=matrix \
	  FEATURES=targets_per_zone_1,ambient_per_spad,nb_spads_enabled,signal_per_spad,range_sigma_mm,distance_mm,reflectance_percent \
	  $(MAKE) _build _run

m2:
	EXAMPLE=m2 \
	  FEATURES=targets_per_zone_2,ambient_per_spad,nb_spads_enabled,signal_per_spad,range_sigma_mm,distance_mm,reflectance_percent \
	  $(MAKE) _build _run

#---
_build:
	$(LOG_ENV) cargo build --timings --release --features=$(FEATURES),$(DEFMT_FEATURES) --example $(EXAMPLE)

_run:
	probe-rs run --speed=200 --log-format $(DEFMT_HOST_FMT) target/$(TARGET)/release/examples/$(EXAMPLE)

echo:
	@echo a

.PHONY: all _nada m matrix matrix-build _build _run echo
