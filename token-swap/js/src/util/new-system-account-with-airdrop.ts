// @flow

import {Account, Connection} from '@gemachain/web3.js';

/**
 * Create a new system account and airdrop it some carats
 *
 * @private
 */
export async function newSystemAccountWithAirdrop(
  connection: Connection,
  carats: number = 1,
): Promise<Account> {
  const account = new Account();
  await connection.requestAirdrop(account.publicKey, carats);
  return account;
}
