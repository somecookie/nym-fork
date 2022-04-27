import type { Coin } from './Coin';

export interface Balance {
  coin: Coin;
  printable_balance: string;
}
