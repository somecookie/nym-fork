import type { MajorCurrencyAmount } from './Currency';
import type { MixNode } from './Mixnode';

export interface MixNodeBond {
  pledge_amount: MajorCurrencyAmount;
  total_delegation: MajorCurrencyAmount;
  owner: String;
  layer: String;
  block_height: bigint;
  mix_node: MixNode;
  proxy?: String;
  accumulated_rewards: MajorCurrencyAmount | null;
}
