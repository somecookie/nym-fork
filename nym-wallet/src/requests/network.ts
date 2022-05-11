import { Account, Network } from '@nymproject/types';
import { invokeWrapper } from './wrapper';

export const selectNetwork = async (network: Network) => invokeWrapper<Account>('switch_network', { network });
