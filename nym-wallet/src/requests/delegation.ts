import { TDelegation } from '@nymproject/types';
import { invokeWrapper } from './wrapper';

export const getMixNodeDelegationsForCurrentAccount = async () =>
  invokeWrapper<TDelegation[]>('get_all_mix_delegations');

export const getDelegationSummary = async () => {
  invokeWrapper('get_delegation_summary');
};
