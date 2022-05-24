import type { Delegation } from './Delegation';
import type { MajorCurrencyAmount } from './Currency';

export interface DelegationsSummaryResponse {
  delegations: Array<Delegation>;
  total_delegations: MajorCurrencyAmount;
  total_rewards: MajorCurrencyAmount;
}
