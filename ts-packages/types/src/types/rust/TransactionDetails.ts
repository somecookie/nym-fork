import type { Coin } from './Coin';

export interface TransactionDetails {
  amount: Coin;
  from_address: string;
  to_address: string;
}
