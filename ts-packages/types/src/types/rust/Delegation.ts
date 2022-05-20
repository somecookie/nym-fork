import type { MajorCurrencyAmount } from './Currency';

export interface Delegation {
  owner: String;
  node_identity: string;
  amount: MajorCurrencyAmount;
  block_height: bigint;
  proxy?: String;
}
