import type { VestingPeriod } from './VestingPeriod';
import { MajorCurrencyAmount } from './Currency';

export interface VestingAccountInfo {
  owner_address: string;
  staking_address: string | null;
  start_time: bigint;
  periods: Array<VestingPeriod>;
  coin: MajorCurrencyAmount;
}
