// @flow

import {sendAndConfirmTransaction as realSendAndConfirmTransaction} from '@gemachain/web3.js';
import type {
  Connection,
  Signer,
  Transaction,
  TransactionSignature,
} from '@gemachain/web3.js';

export function sendAndConfirmTransaction(
  title: string,
  connection: Connection,
  transaction: Transaction,
  ...signers: Array<Signer>
): Promise<TransactionSignature> {
  return realSendAndConfirmTransaction(connection, transaction, signers, {
    skipPreflight: false,
  });
}
