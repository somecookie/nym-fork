import type { Coin } from './Coin';

export interface PledgeData {
  amount: Coin;
  block_time: bigint;
}
