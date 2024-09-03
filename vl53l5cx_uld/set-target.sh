#!/bin/bash
set -e

#
# .cargo/config.toml:
#     ^target = "riscv32imc-unknown-none-elf"    | ESP32-C3
#     ^target = "riscv32imac-unknown-none-elf"   | ESP32-C6
#
# Cargo.toml:
#     "esp32c6"
#     "esp32c3"
#

# Detect which MCU the system is currently tuned for
#
if grep -q '"esp32c3"' Cargo.toml; then
  MCU=esp32c3
elif grep -q '"esp32c6"' Cargo.toml; then
  MCU=esp32c6
else
  (echo >&2 "Error parsing 'Cargo.toml'; please set up manually!"; false)
fi

# Ask interactively
#
# NOTE: Currently NOT using the 'current' MCU, but could (if we use more advanced UI that's able to highlight
#     the active selection. tbd.)
#
options=("esp32c3" "esp32c6")

PS3="Pick your target: "
select opt in "${options[@]}"; do
  case "$REPLY" in
    1) MCU=esp32c3; break;;
    2) MCU=esp32c6; break;;
    *) exit 50;;
  esac
done

echo ""
echo "'${MCU}' selected."
echo ""

read -n1 -p "Continue? (Y/n) " INPUT
if ! echo $INPUT | grep '^[Yy]\?$'; then
  echo ''
  exit 1
fi

# TARGET matching the chip
#
case "$MCU" in
  esp32c3) TARGET=riscv32imc-unknown-none-elf ;;
  esp32c6) TARGET=riscv32imac-unknown-none-elf ;;
  *) (echo >&2 "Unexpected MCU=${MCU}"; exit 50) ;;
esac

# Check here (before we make any changes) that there is a pin line in the examples, for this MCU.
#
#   Original attempt:
#     $ grep -Eq "^\s+(?://)?\(io\.pins\..+\)\s*//\s*${MCU}\s*$"
#       macOS: 👍
#       Linux: grep: warning: ? at start of expression
#
#   Solution:
#     - do NOT use non-capturing group '(?:...)', but only capturing ones '(...)'; works on both OSes.
#
(cat examples/_2-get_set_parameters.rs | \
  grep -Eq "^\s+(//)?\(io\.pins\..+\)\s*//\s*${MCU}\s*$" \
  ) || (
  echo >&2 "ERROR: Did NOT find a line for the pins, for ${MCU}."; false
)

# Modify the files, to anchor the selection
#
# Note: we don't need backups since the files are (presumably) version controlled, anyhow.
#
# Dev note:
#   'sed' _does_ have '-i' ("in place editing"), but we can do without. It's a bit hairy; piping just feels nicer!!!
#
# macOS note:
#   '\s' did not work; '[[:space:]]' does
#     -> https://superuser.com/questions/112834/how-to-match-whitespace-in-sed
#
cp .cargo/config.toml tmp-1
cat tmp-1 | sed -E "s/^(target[[:space:]]*=[[:space:]]*\")riscv32im[a]?c\-unknown\-none\-elf(\".+)$/\1${TARGET}\2/g" \
  > .cargo/config.toml

cp Cargo.toml tmp-2
cat tmp-2 | sed -E "s/(\")esp32c[36](\")/\1${MCU}\2/g" \
  > Cargo.toml

# Take such lines, and enable the one with the '// {MCU}' in the trailing comment
#   <<
#         (io.pins.gpio4, io.pins.gpio5, Some(io.pins.gpio0), NO_PIN)      // esp32c3
#         //(io.pins.gpio22, io.pins.gpio23, Some(io.pins.gpio21), NO_PIN)    // esp32c6
#   <<
#
# Note: Why do we do it like this? Features are the normal way to go, but the author wanted to ... not use them ...
#     mainly since they are only needed for our *examples* but defining them would (does it, with "resolver" 2) infect
#     the library as well. The library is *agnostic* of architectures. IF you suggest something else, consider that
#     first.    tbd. consider moving to 'DEVS/No\ to\ features.md'
#
cp examples/_2-get_set_parameters.rs tmp-3
cat tmp-3 \
  | sed -E 's_^([[:space:]]+)(//)?(\(io\.pins\..+\)[[:space:]]*//[[:space:]]*esp32.+$)_\1//\3_g' \
  | sed -E "s_^([[:space:]]+)//(\(io\.pins\..+\)[[:space:]]*//[[:space:]]*${MCU})[[:space:]]*\$_\1\2_g" \
  > examples/_2-get_set_parameters.rs

rm tmp-[123]

echo "Files '.cargo/config.toml' and 'Cargo.toml' now using:"
echo ""
echo "   MCU:    ${MCU}"
echo "   TARGET: ${TARGET}"
echo ""
echo "Please 'cargo build' or 'cargo run', as usual."
echo ""
