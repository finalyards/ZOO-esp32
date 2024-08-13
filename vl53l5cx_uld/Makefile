#
# This Makefile is only for help - a wrapper around 'cargo build' to shorten the commands.
#
# Usage:
#	$ make {0-nada|...}
#		# builds and runs a certain sample
#
PROFILE?=release
DEFMT_LOG:=debug

LOG_ENV:=DEFMT_LOG=$(DEFMT_LOG)

ifeq (dev,$(PROFILE))
--PROFILE:=--profile dev
else ifeq (release,$(PROFILE))
--PROFILE:=--release
else
  $(error Unknown 'PROFILE': "$(PROFILE)")
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
	$(LOG_ENV) cargo run $(--PROFILE) --features=defmt --example $@

#TEMP
ranging-basic: _1-ranging_basic

$(EXAMPLE_NAMES):
	$(LOG_ENV) cargo run $(--PROFILE) --features=defmt --example $@

_clean:
	-cargo clean

echo:
	@echo $(EXAMPLE_NAMES)

.PHONY: all lib $(EXAMPLE_NAMES) _clean echo
