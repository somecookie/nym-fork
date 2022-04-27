import type { Coin } from './Coin';
import type { VestingPeriod } from './VestingPeriod';

export interface VestingAccountInfo {
  owner_address: string;
  staking_address: string | null;
  start_time: bigint;
  periods: Array<VestingPeriod>;
  coin: Coin;
}
