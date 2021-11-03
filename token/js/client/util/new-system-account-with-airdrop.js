// @flow

import {Keypair, Connection, Signer} from '@gemachain/web3.js';

/**
 * Create a new system account and airdrop it some carats
 *
 * @private
 */
export async function newSystemAccountWithAirdrop(
  connection: Connection,
  carats: number = 1,
): Promise<Signer> {
  const account = Keypair.generate();
  await connection.requestAirdrop(account.publicKey, carats);
  return account;
}
