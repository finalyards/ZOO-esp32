#
#
#

#_MCU=esp32c3
_MCU=esp32c6

ifneq (,$(findstring $(_MCU),esp32c3))
_TARGET=riscv32imc-unknown-none-elf
else ifneq (,$(findstring $(_MCU),esp32c6))
_TARGET=riscv32imac-unknown-none-elf
else
$(error Unexpected MCU: $(_MCU))
endif

# 0 (debug) | 1 (release)
RELEASE?=0

_DEF=def

ifeq (1,$(RELEASE))
  _RELEASE_OR_DEBUG:=release
  _RELEASE_FLAG:=--release
else
  _RELEASE_OR_DEBUG:=debug
  _RELEASE_FLAG:=
endif

_CARGO_PACKAGE_NAME:=vl53l5cx

# The output file 'make build --always-make' creates
_X:=xxx

_SRC:=src/**/*.rs
_CFG:=.cargo/config.toml Cargo.toml build.rs

all:
	@echo "Usage:"
	@echo "  [RELEASE=1] make build"
	@echo "  [RELEASE=1] make run	  // runs the already compiled code"
	@echo ""
	@false

build: $(_X)

$(_X): $(_SRC) $(_CFG) uld-sys Makefile
	@# Makefile in deps, so that change in '_MCU' and '_TARGET' are accounted for
	cargo build \
	  $(_RELEASE_FLAG) \
	  --features $(_MCU) \
	  --target $(_TARGET)

run!:
	espflash flash --monitor target/$(_TARGET)/$(_RELEASE_OR_DEBUG)/$(_CARGO_PACKAGE_NAME)

#R##-- Embassy examples (for checking toolchain works)
#R#
#R_hello-build:
#R	(cd $(_DEF) && cargo build --bin embassy_hello_world \
#R	  $(_RELEASE_FLAG) \
#R	  --features $(_MCU),embassy,esp-hal-embassy/integrated-timers \
#R	  --target $(_TARGET) \
#R	)
#R
#R# Run the Embassy hello example
#R#
#R# Note: Also 'cargo run' would do the same (but check that the build is up-to-date, taking time)
#R#
#R_hello-run!:
#R	(cd $(_DEF) && \
#R	  espflash flash --monitor target/$(_TARGET)/$(_RELEASE_OR_DEBUG)/embassy_hello_world \
#R	)

echo:
	@echo $(_RELEASE_OR_DEBUG)

.PHONY: all echo
