#!/usr/bin/env bash

# Script to deposit and withdraw stakes from a pool, given stake pool public key
# and a path to a file containing a list of validator vote accounts

cd "$(dirname "$0")"
stake_pool_keyfile=$1
validator_list=$2

create_keypair () {
  if test ! -f $1
  then
    gemachain-keygen new --no-passphrase -s -o $1
  fi
}

create_user_stakes () {
  validator_list=$1
  gema_amount=$2
  authority=$3
  for validator in $(cat $validator_list)
  do
    create_keypair $keys_dir/stake_$validator.json
    gemachain create-stake-account $keys_dir/stake_$validator.json $gema_amount --withdraw-authority $authority --stake-authority $authority
  done
}

delegate_user_stakes () {
  validator_list=$1
  authority=$2
  for validator in $(cat $validator_list)
  do
    gemachain delegate-stake --force $keys_dir/stake_$validator.json $validator --stake-authority $authority
  done
}

deposit_stakes () {
  stake_pool_pubkey=$1
  validator_list=$2
  authority=$3
  for validator in $(cat $validator_list)
  do
    stake=$(gemachain-keygen pubkey $keys_dir/stake_$validator.json)
    $spl_stake_pool deposit-stake $stake_pool_pubkey $stake --withdraw-authority $authority
  done
}

withdraw_stakes () {
  stake_pool_pubkey=$1
  validator_list=$2
  pool_amount=$3
  for validator in $(cat $validator_list)
  do
    $spl_stake_pool withdraw-stake $stake_pool_pubkey $pool_amount --vote-account $validator
  done
}

gema_amount=2
half_gema_amount=1
keys_dir=keys
spl_stake_pool=../../../target/debug/spl-stake-pool
stake_pool_pubkey=$(gemachain-keygen pubkey $stake_pool_keyfile)
echo "Setting up keys directory $keys_dir"
mkdir -p $keys_dir
authority=$keys_dir/authority.json
echo "Setting up authority for deposited stake accounts at $authority"
create_keypair $authority

echo "Creating user stake accounts to deposit into the pool"
create_user_stakes $validator_list $gema_amount $authority
echo "Delegating user stakes so that deposit will work"
delegate_user_stakes $validator_list $authority
echo "Waiting for stakes to activate, this may take awhile depending on the network!"
echo "If you are running on localnet with 32 slots per epoch, wait 12 seconds..."
sleep 12
echo "Depositing stakes into stake pool"
deposit_stakes $stake_pool_pubkey $validator_list $authority
echo "Withdrawing stakes from stake pool"
withdraw_stakes $stake_pool_pubkey $validator_list $half_gema_amount
echo "Depositing GEMA into stake pool to authority"
$spl_stake_pool deposit-gema $stake_pool_pubkey $gema_amount
echo "Withdrawing GEMA from stake pool to authority"
$spl_stake_pool withdraw-gema $stake_pool_pubkey $authority $half_gema_amount
