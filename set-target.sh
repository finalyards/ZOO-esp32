#!/bin/bash
set -e

#
# Changes the target MCU _based on the current working directory_.
#
#   - looks for '.cargo/config.toml' in this folder. Modifies its 'target' entry to match the MCU.
#       ^target = "riscv32imc-unknown-none-elf"    | ESP32-C3
#       ^target = "riscv32imac-unknown-none-elf"   | ESP32-C6
#   - looks for 'Cargo.toml' in this _and any subfolders_, changing their features:
#       "esp32c6"
#       "esp32c3"
#

# Expect '.cargo/config.toml' under the current folder.
#
CONFIG_TOML=.cargo/config.toml
test -f $CONFIG_TOML || (
  echo >&2 "ERROR: Did not find '$CONFIG_TOML'; please run in a folder that has such."; false
)

# Find the 'Cargo.toml' file(s) that would get treated.
#
CARGO_TOMLS=$(find . -name Cargo.toml)
  # e.g. "./drv8871/Cargo.toml [...]"

test -n "${CARGO_TOMLS}" || (
  echo >&2 "ERROR: Did not find any 'Cargo.toml' in this, or subfolders. This is strange."; false
)

ONE_CARGO_TOML=${CARGO_TOMLS[0]}

# Detect which MCU the system is currently tuned for.
#
# NOTE: This needs to happen based on a 'Cargo.toml' since multiple MCU's may (do/will) share the same 'target' string.
# NOTE 2: This is NOT USED YET; the idea is to have the current selection as a default for the prompt.
#
if grep -q '"esp32c3"' $ONE_CARGO_TOML; then
  _MCU_NOT_USED=esp32c3
elif grep -q '"esp32c6"' $ONE_CARGO_TOML; then
  _MCU_NOT_USED=esp32c6
else
  echo >&2 "Error parsing '$ONE_CARGO_TOML'; please set up manually!"; false
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
    *) false;;
  esac
done

echo ""
echo "'${MCU}' selected."
echo ""
echo "Going to edit:"
echo "- ${CONFIG_TOML}"
echo "- ${CARGO_TOMLS}"
echo ""
echo "The edit is safe. You can see the changes made by 'git diff' (and revert by 'git restore --staged ${CONFIG_TOML} ${CARGO_TOMLS}')."
echo ""
read -n1 -p "Proceed? (Y/n) " INPUT
if ! echo $INPUT | grep '^[Yy]\?$'; then
  echo ''; false
fi

# TARGET matching the selected chip
#
case "$MCU" in
  esp32c3) TARGET=riscv32imc-unknown-none-elf ;;
  esp32c6) TARGET=riscv32imac-unknown-none-elf ;;
  *) (echo >&2 "Unexpected MCU: '${MCU}'"; false) ;;
esac

# Modify the files, to anchor the selection
#
# Note: we don't need backups since the files are (presumably) version controlled.
#
# Dev note:
#   'sed' _does_ have '-i' ("in place editing"), but we can do without. It's a bit hairy; piping just feels nicer!!!
#
# macOS note:
#   '\s' did not work; '[[:space:]]' does
#     -> https://superuser.com/questions/112834/how-to-match-whitespace-in-sed
#
cp ${CONFIG_TOML} tmp-1
cat tmp-1 | sed -E "s/^(target[[:space:]]*=[[:space:]]*\")riscv32im[a]?c\-unknown\-none\-elf(\".+)$/\1${TARGET}\2/g" \
  > ${CONFIG_TOML}

for TOML in "${CARGO_TOMLS}"
do
  cp ${TOML} tmp-2
  cat tmp-2 | sed -E "s/(\")esp32c[36](\")/\1${MCU}\2/g" \
    > ${TOML}
done

rm tmp-[12]

echo "Files '${CONFIG_TOML}' and '${CARGO_TOMLS}' now using:"
echo ""
echo "   MCU:    ${MCU}"
echo "   TARGET: ${TARGET}"
echo ""
echo "Please 'cargo build' or 'cargo run', as usual."
echo ""
