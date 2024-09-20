#!/bin/bash
set -e

#
# Usage:
#   <<
#     $ ./set-address.sh [old-addr-hex] new-addr-hex     # e.g. ./set-address.sh 0x52 0x56
#   <<
#
# Writes the change to the VL53L5CX chip, reached via 'probe-rs'.
#
OLD_ADDR=0x52
NEW_ADDR=

case $# in
  0)
    echo >&2 -e "Usage:\n  $0 [0x52] 0x54\n"
    exit 0
    ;;
  1)
    NEW_ADDR=$1
    ;;
  2)
    OLD_ADDR=$1
    NEW_ADDR=$2
    ;;
esac

# Validate that given values are hex
#
arr=("$OLD_ADDR" "$NEW_ADDR")
for s in "${arr[@]}"; do
  # bash note: Bash internal regex was a chore, with '^' and hex. Easier to retain to external 'grep -E'.
  (echo $s | grep -qE '^0x[[:xdigit:]]{2}$') || (
    echo >&2 "ERROR: address doesn't seem to be valid (hex with '0x' prefix; e.g. '0x56'): $s"; false
  )
done

NAME=set-address
TARGET=$(cat .cargo/config.toml | grep -e '^target\s*=\s"' | cut -d '"' -f2)
  # riscv32imac-unknown-none-elf

# Roughly inject the parameters to the source code to be compiled!
#
# Yes, we'd prefer to use 'semihosting', but probe-rs (0.24.0; Sep'24) doesn't support any of:
#   - args
#   - stdio
#   - fs
#
# Since we ARE hopeful that 'args' is eventually supported, consider this just an interim hack.
#
cp examples/set-address.rs tmp-1
cat tmp-1 \
  | sed -E "s/^(const\\s+OLD_ADDR:.+=\\s*)0x[[:xdigit:]]{2}(;.*)\$/\1${OLD_ADDR}\2/" \
  | sed -E "s/^(const\\s+NEW_ADDR:.+=\\s*)0x[[:xdigit:]]{2}(;.*)\$/\1${NEW_ADDR}\2/" \
  > examples/set-address.rs

rm tmp-1

# Run on the device, using 'probe-rs' and 'semihosting'
#
# https://stackoverflow.com/a/64644990/8608146
exe(){
    set -x
    "$@"
    { set +x; } 2>/dev/null
}

export DEFMT_LOG=debug  # hack (cannot set the value just for one line, because of 'exe' function)
exe cargo build --release --features=defmt --example ${NAME}
unset DEFMT_LOG; export DEFMT_LOG

exe probe-rs run --log-format '{t:dimmed} [{L:bold}] {s}' target/${TARGET}/release/examples/${NAME}
