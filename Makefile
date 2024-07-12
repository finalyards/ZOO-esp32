#
#
#

# C3
#_CHIP=esp32c3
#_TARGET=riscv32imc-unknown-none-elf

# C6
_CHIP=esp32c6
_TARGET=riscv32imac-unknown-none-elf

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

all:
	@echo "Usage:"
	@echo "  [RELEASE=1] make build"
	@echo "  [RELEASE=1] make run	  // runs the already compiled code"
	@echo ""
	@false

build:
	cargo build \
	  $(_RELEASE_FLAG) \
	  --features $(_CHIP),embassy,esp-hal-embassy/integrated-timers \
	  --target $(_TARGET)

run!:
	espflash flash --monitor target/$(_TARGET)/$(_RELEASE_OR_DEBUG)/xxx

##-- Embassy examples (for checking toolchain works)
#
_hello-build:
	(cd $(_DEF) && cargo build --bin embassy_hello_world \
	  $(_RELEASE_FLAG) \
	  --features $(_CHIP),embassy,esp-hal-embassy/integrated-timers \
	  --target $(_TARGET) \
	)

# Run the Embassy hello example
#
# Note: Also 'cargo run' would do the same (but check that the build is up-to-date, taking time)
#
_hello-run!:
	(cd $(_DEF) && \
	  espflash flash --monitor target/$(_TARGET)/$(_RELEASE_OR_DEBUG)/embassy_hello_world \
	)

echo:
	@echo $(_RELEASE_OR_DEBUG)

.PHONY: all echo
