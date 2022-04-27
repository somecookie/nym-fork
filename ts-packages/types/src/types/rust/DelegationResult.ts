import type { Coin } from './Coin';

export interface DelegationResult {
  source_address: string;
  target_address: string;
  amount: Coin | null;
}
