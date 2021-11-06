#!/usr/bin/env bash
#
# Patches the GPL crates for developing against a local gemachain monorepo
#

gemachain_dir=$1
if [[ -z $gemachain_dir ]]; then
  echo "Usage: $0 <path-to-gemachain-monorepo>"
  exit 1
fi

workspace_crates=(
  Cargo.toml
  themis/client_ristretto/Cargo.toml
)

if [[ ! -r "$gemachain_dir"/scripts/read-cargo-variable.sh ]]; then
  echo "$gemachain_dir is not a path to the gemachain monorepo"
  exit 1
fi

set -e

gemachain_dir=$(cd "$gemachain_dir" && pwd)
cd "$(dirname "$0")"

source "$gemachain_dir"/scripts/read-cargo-variable.sh
gemachain_ver=$(readCargoVariable version "$gemachain_dir"/sdk/Cargo.toml)

echo "Patching in $gemachain_ver from $gemachain_dir"
echo
for crate in "${workspace_crates[@]}"; do
  if grep -q '\[patch.crates-io\]' "$crate"; then
    echo "$crate is already patched"
  else
    cat >> "$crate" <<PATCH
[patch.crates-io]
gemachain-account-decoder = {path = "$gemachain_dir/account-decoder" }
gemachain-banks-client = { path = "$gemachain_dir/banks-client"}
gemachain-banks-server = { path = "$gemachain_dir/banks-server"}
gemachain-bpf-loader-program = { path = "$gemachain_dir/programs/bpf_loader" }
gemachain-clap-utils = {path = "$gemachain_dir/clap-utils" }
gemachain-cli-config = {path = "$gemachain_dir/cli-config" }
gemachain-cli-output = {path = "$gemachain_dir/cli-output" }
gemachain-client = { path = "$gemachain_dir/client"}
gemachain-core = { path = "$gemachain_dir/core"}
gemachain-logger = {path = "$gemachain_dir/logger" }
gemachain-notifier = { path = "$gemachain_dir/notifier" }
gemachain-remote-wallet = {path = "$gemachain_dir/remote-wallet" }
gemachain-program = { path = "$gemachain_dir/sdk/program" }
gemachain-program-test = { path = "$gemachain_dir/program-test" }
gemachain-runtime = { path = "$gemachain_dir/runtime" }
gemachain-sdk = { path = "$gemachain_dir/sdk" }
gemachain-stake-program = { path = "$gemachain_dir/programs/stake" }
gemachain-transaction-status = { path = "$gemachain_dir/transaction-status" }
gemachain-vote-program = { path = "$gemachain_dir/programs/vote" }
PATCH
  fi
done

./update-gemachain-dependencies.sh "$gemachain_ver"
