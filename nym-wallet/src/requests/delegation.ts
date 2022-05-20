import { invokeWrapper } from './wrapper';
import { TDelegation } from '@nymproject/types';

export const getMixNodeDelegationsForCurrentAccount = async () =>
  invokeWrapper<TDelegation[]>('get_all_mix_delegations');
