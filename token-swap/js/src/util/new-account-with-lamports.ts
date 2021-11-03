// @flow

import {Account, Connection} from '@gemachain/web3.js';

import {sleep} from './sleep';

export async function newAccountWithCarats(
  connection: Connection,
  carats: number = 1000000,
): Promise<Account> {
  const account = new Account();

  let retries = 30;
  await connection.requestAirdrop(account.publicKey, carats);
  for (;;) {
    await sleep(500);
    if (carats == (await connection.getBalance(account.publicKey))) {
      return account;
    }
    if (--retries <= 0) {
      break;
    }
  }
  throw new Error(`Airdrop of ${carats} failed`);
}
