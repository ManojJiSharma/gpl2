#!/usr/bin/env bash
#
# Updates the gemachain version in all the GPL crates
#

gemachain_ver=$1
if [[ -z $gemachain_ver ]]; then
  echo "Usage: $0 <new-gemachain-version>"
  exit 1
fi

cd "$(dirname "$0")"

sed -i'' -e "s#gemachain_version=v.*#gemachain_version=v${gemachain_ver}#" ./ci/gemachain-version.sh

declare tomls=()
while IFS='' read -r line; do tomls+=("$line"); done < <(find . -name Cargo.toml)

crates=(
  gemachain-account-decoder
  gemachain-banks-client
  gemachain-banks-server
  gemachain-bpf-loader-program
  gemachain-clap-utils
  gemachain-cli-config
  gemachain-cli-output
  gemachain-client
  gemachain-core
  gemachain-logger
  gemachain-notifier
  gemachain-program
  gemachain-program-test
  gemachain-remote-wallet
  gemachain-runtime
  gemachain-sdk
  gemachain-stake-program
  gemachain-transaction-status
  gemachain-vote-program
)

set -x
for crate in "${crates[@]}"; do
  sed -i'' -e "s#\(${crate} = \"\)\(=\?\).*\(\"\)#\1\2$gemachain_ver\3#g" "${tomls[@]}"
done
