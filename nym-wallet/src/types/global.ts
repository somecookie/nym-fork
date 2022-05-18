import { CurrencyDenom, Gateway, MajorCurrencyAmount, MixNode, PledgeData } from '@nymproject/types';

export enum EnumNodeType {
  mixnode = 'mixnode',
  gateway = 'gateway',
}

export type TNodeOwnership = {
  hasOwnership: boolean;
  nodeType?: EnumNodeType;
  vestingPledge?: PledgeData;
};

export type TClientDetails = {
  account: {
    client_address: string;
    contract_address: string;
    demon: CurrencyDenom;
  };
};

export type TSignInWithMnemonic = {
  denom: string;
} & TClientDetails;

export type TCreateAccount = {
  mnemonic: string;
} & TSignInWithMnemonic;

export type TFee = {
  [EnumNodeType.mixnode]: MajorCurrencyAmount;
  [EnumNodeType.gateway]?: MajorCurrencyAmount;
};

export type TPendingDelegation = {
  block_height: number;
};

export type TDelegation = {
  owner: string;
  node_identity: string;
  amount: MajorCurrencyAmount;
  block_height: number;
  proxy: string; // proxy address used to delegate the funds on behalf of another address
  pending?: TPendingDelegation;
};

export type TPagedDelegations = {
  delegations: TDelegation[];
  start_next_after: string;
};

export type TMixnodeBondDetails = {
  pledge_amount: MajorCurrencyAmount;
  total_delegation: MajorCurrencyAmount;
  owner: string;
  layer: string;
  block_height: number;
  mix_node: MixNode;
  proxy: any;
};

export type TBondArgs = {
  type: EnumNodeType;
  data: MixNode | Gateway;
  pledge: MajorCurrencyAmount;
  ownerSignature: string;
};

export type TDelegateArgs = {
  type: EnumNodeType;
  identity: string;
  amount: MajorCurrencyAmount;
};

export type Period = 'Before' | { In: number } | 'After';
