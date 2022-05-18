import {
  DelegationResult,
  EnumNodeType,
  Gateway,
  MajorCurrencyAmount,
  MixNode,
  OriginalVestingResponse,
  Period,
  PledgeData,
  VestingAccountInfo,
} from '@nymproject/types';
import { invokeWrapper } from './wrapper';

export const getLockedCoins = async (): Promise<MajorCurrencyAmount> =>
  invokeWrapper<MajorCurrencyAmount>('locked_coins');

export const getSpendableCoins = async (): Promise<MajorCurrencyAmount> =>
  invokeWrapper<MajorCurrencyAmount>('spendable_coins');

export const getVestingCoins = async (vestingAccountAddress: string): Promise<MajorCurrencyAmount> =>
  invokeWrapper<MajorCurrencyAmount>('vesting_coins', { vestingAccountAddress });

export const getVestedCoins = async (vestingAccountAddress: string): Promise<MajorCurrencyAmount> =>
  invokeWrapper<MajorCurrencyAmount>('vested_coins', { vestingAccountAddress });

export const getOriginalVesting = async (vestingAccountAddress: string): Promise<OriginalVestingResponse> => {
  const res = await invokeWrapper<OriginalVestingResponse>('original_vesting', { vestingAccountAddress });
  return { ...res, amount: res.amount };
};

export const withdrawVestedCoins = async (amount: string, denom: MajorCurrencyAmount): Promise<void> => {
  await invokeWrapper('withdraw_vested_coins', { amount: { amount, denom } });
};

export const getCurrentVestingPeriod = async (address: string) =>
  invokeWrapper<Period>('get_current_vesting_period', { address });

export const vestingBond = async ({
  type,
  data,
  pledge,
  ownerSignature,
}: {
  type: EnumNodeType;
  data: MixNode | Gateway;
  pledge: MajorCurrencyAmount;
  ownerSignature: string;
}) => invokeWrapper<void>(`vesting_bond_${type}`, { [type]: data, ownerSignature, pledge });

export const vestingUnbond = async (type: EnumNodeType) => invokeWrapper<void>(`vesting_unbond_${type}`);

export const vestingDelegateToMixnode = async ({
  identity,
  amount,
}: {
  identity: string;
  amount: MajorCurrencyAmount;
}) => invokeWrapper<DelegationResult>('vesting_delegate_to_mixnode', { identity, amount });

export const vestingUnelegateFromMixnode = async (identity: string) =>
  invokeWrapper<DelegationResult>('vesting_undelegate_from_mixnode', { identity });

export const getVestingAccountInfo = async (address: string) =>
  invokeWrapper<VestingAccountInfo>('get_account_info', { address });

export const getVestingPledgeInfo = async ({
  address,
  type,
}: {
  address?: string;
  type: EnumNodeType;
}): Promise<PledgeData | undefined> => {
  try {
    return await invokeWrapper<PledgeData>(`vesting_get_${type}_pledge`, { address });
  } catch (e) {
    return undefined;
  }
};

export const vestingUpdateMixnode = async (profitMarginPercent: number) =>
  invokeWrapper<void>('vesting_update_mixnode', { profitMarginPercent });

export const vestingDelegatedFree = async (vestingAccountAddress: string) =>
  invokeWrapper<MajorCurrencyAmount>('delegated_free', { vestingAccountAddress });
