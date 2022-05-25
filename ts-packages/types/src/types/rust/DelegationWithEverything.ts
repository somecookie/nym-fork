import type { MajorCurrencyAmount } from './Currency';

export interface DelegationWithEverything {
  owner: string;
  node_identity: string;
  amount: MajorCurrencyAmount;
  block_height: bigint;
  delegated_on_iso: string;
  profit_margin_percent: number;
  stake_saturation: number;
  proxy: string | null;
  accumulated_rewards: MajorCurrencyAmount | null;
}
