import type { Denom } from './Denom';

export interface Coin {
  amount: string;
  denom: Denom;
}
