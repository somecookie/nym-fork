import type { CurrencyDenom } from './CurrencyDenom';
import type { StringMajorAmount } from './CurrencyStringMajorAmount';

export interface MajorCurrencyAmount {
  amount: StringMajorAmount;
  denom: CurrencyDenom;
}
