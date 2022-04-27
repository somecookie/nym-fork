import type { Denom } from './Denom';

export interface Account {
  contract_address: string;
  client_address: string;
  denom: Denom;
}
