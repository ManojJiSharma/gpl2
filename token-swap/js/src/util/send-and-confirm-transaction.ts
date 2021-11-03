import {sendAndConfirmTransaction as realSendAndConfirmTransaction} from '@gemachain/web3.js';
import type {
  Account,
  Connection,
  Transaction,
  TransactionSignature,
} from '@gemachain/web3.js';

export function sendAndConfirmTransaction(
  title: string,
  connection: Connection,
  transaction: Transaction,
  ...signers: Array<Account>
): Promise<TransactionSignature> {
  return realSendAndConfirmTransaction(connection, transaction, signers, {
    skipPreflight: false,
    commitment: 'recent',
    preflightCommitment: 'recent',
  });
}
