import React, { createContext, FC, useCallback, useContext, useEffect, useMemo, useState } from 'react';
import type { Network } from 'src/types';
import type { DelegateListItem } from '../components/Delegation/types';

export type TDelegationContext = {
  isLoading: boolean;
  error?: string;
  delegations?: DelegateListItem[];
  totalDelegations?: string;
  refresh: () => Promise<void>;
  getDelegations: () => Promise<DelegateListItem[]>;
  addDelegation: (newDelegation: DelegateListItem) => Promise<TDelegationTransaction>;
  updateDelegation: (newDelegation: DelegateListItem) => Promise<TDelegationTransaction>;
  undelegate: (mixnodeAddress: string) => Promise<TDelegationTransaction>;
};

export type TDelegationTransaction = {
  transactionUrl: string;
};

export const DelegationContext = createContext<TDelegationContext>({
  isLoading: true,
  refresh: async () => undefined,
  getDelegations: async () => [
    {
      id: 'FiojKW7oY9WQmLCiYAsCA21tpowZHS6zcUoyYm319p6Z',
      delegationDate: new Date(2021, 1, 1),
      amount: '452 NYM',
      uptimePercentage: 0.832,
      profitMarginPercentage: 0.1122323949234,
      reward: '0.001523434 NYM',
    },
    {
      id: 'DT8S942S8AQs2zKHS9SVo1GyHmuca3pfL2uLhLksJ3D8',
      delegationDate: new Date(2021, 1, 2),
      amount: '1000000 NYM',
      uptimePercentage: 0.2323423424,
      profitMarginPercentage: 0.1,
    },
    {
      id: '6hn3z2yCQ3KP8XyqMRMV4c6DvYWG1vvrAWpgkxe1CV9C',
      delegationDate: new Date(2021, 1, 3),
      amount: '1 NYM',
      uptimePercentage: 1.0,
      profitMarginPercentage: 0.11,
      reward: '0.00156 NYM',
    },
  ],
  addDelegation: async () => {
    throw new Error('Not implemented');
  },
  updateDelegation: async () => {
    throw new Error('Not implemented');
  },
  undelegate: async () => {
    throw new Error('Not implemented');
  },
});

export const DelegationContextProvider: FC<{
  network?: Network;
}> = ({ network, children }) => {
  const [currentNetwork, setCurrentNetwork] = useState<undefined | Network>();
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string>();
  const [delegations, setDelegations] = useState<undefined | DelegateListItem[]>();
  const [totalDelegations, setTotalDelegations] = useState<undefined | string>();

  // TODO: implement
  const getDelegations = async (): Promise<DelegateListItem[]> => [
    {
      id: 'FiojKW7oY9WQmLCiYAsCA21tpowZHS6zcUoyYm319p6Z',
      delegationDate: new Date(2021, 1, 1),
      amount: '452 NYM',
      uptimePercentage: 0.832,
      profitMarginPercentage: 0.1122323949234,
      reward: '0.001523434 NYM',
    },
    {
      id: 'DT8S942S8AQs2zKHS9SVo1GyHmuca3pfL2uLhLksJ3D8',
      delegationDate: new Date(2021, 1, 2),
      amount: '1000000 NYM',
      uptimePercentage: 0.2323423424,
      profitMarginPercentage: 0.1,
    },
    {
      id: '6hn3z2yCQ3KP8XyqMRMV4c6DvYWG1vvrAWpgkxe1CV9C',
      delegationDate: new Date(2021, 1, 3),
      amount: '1 NYM',
      uptimePercentage: 1.0,
      profitMarginPercentage: 0.11,
      reward: '0.00156 NYM',
      isPending: { actionType: 'delegate', blockHeight: 1 },
    },
  ];

  const addDelegation = async (): Promise<TDelegationTransaction> => {
    throw new Error('Not implemented');
  };
  const updateDelegation = async (): Promise<TDelegationTransaction> => {
    throw new Error('Not implemented');
  };
  const undelegate = async (): Promise<TDelegationTransaction> => {
    throw new Error('Not implemented');
  };

  const resetState = () => {
    setIsLoading(true);
    setError(undefined);
    setTotalDelegations(undefined);
    setDelegations([]);
  };

  const refresh = useCallback(async () => {
    try {
      const delegations = await getDelegations();

      setDelegations(delegations);
      setTotalDelegations('500 NYM');
    } catch (e) {
      setError((e as Error).message);
    }
    setIsLoading(false);
  }, [network]);

  useEffect(() => {
    if (currentNetwork !== network) {
      // reset state and refresh
      resetState();
      setCurrentNetwork(network);
      refresh();
    }
  }, [network]);

  const memoizedValue = useMemo(
    () => ({
      isLoading,
      error,
      delegations,
      totalDelegations,
      refresh,
      getDelegations,
      addDelegation,
      updateDelegation,
      undelegate,
    }),
    [isLoading, error, delegations],
  );

  return <DelegationContext.Provider value={memoizedValue}>{children}</DelegationContext.Provider>;
};

export const useDelegationContext = () => useContext<TDelegationContext>(DelegationContext);
