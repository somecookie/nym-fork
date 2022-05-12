import { useContext, useEffect, useState } from 'react';
import { ClientContext } from '../../context/main';
import { useSnackbar } from 'notistack';
import { getInclusionProbability, getMixnodeStakeSaturation, getMixnodeStatus } from '../../requests';
import { InclusionProbabilityResponse, MixnodeStatus } from '@nymproject/types';

export const useSettingsState = (shouldUpdate: boolean) => {
  const [status, setStatus] = useState<MixnodeStatus>('not_found');
  const [saturation, setSaturation] = useState<number>(0);
  const [rewardEstimation, setRewardEstimation] = useState<number>(0);
  const [inclusionProbability, setInclusionProbability] = useState<InclusionProbabilityResponse>({
    in_active: 'VeryLow',
    in_reserve: 'VeryLow',
  });

  const { mixnodeDetails } = useContext(ClientContext);
  const { enqueueSnackbar } = useSnackbar();

  const getStatus = async (mixnodeKey: string) => {
    const newStatus = await getMixnodeStatus(mixnodeKey);
    setStatus(newStatus.status);
  };

  const getStakeSaturation = async (mixnodeKey: string) => {
    const newSaturation = await getMixnodeStakeSaturation(mixnodeKey);

    if (newSaturation) {
      setSaturation(Math.round(newSaturation.saturation * 100));
    }
  };

  const getMixnodeInclusionProbability = async (mixnodeKey: string) => {
    const probability = await getInclusionProbability(mixnodeKey);
    if (probability) {
      setInclusionProbability({ in_active: probability.in_active, in_reserve: probability.in_reserve });
    }
  };

  const reset = () => {
    setStatus('not_found');
    setSaturation(0);
    setRewardEstimation(0);
    setInclusionProbability({ in_active: 'VeryLow', in_reserve: 'VeryLow' });
  };

  useEffect(() => {
    if (shouldUpdate && mixnodeDetails?.mix_node.identity_key) {
      (async () => {
        try {
          await getStatus(mixnodeDetails?.mix_node.identity_key);
          await getStakeSaturation(mixnodeDetails?.mix_node.identity_key);
          await getMixnodeInclusionProbability(mixnodeDetails?.mix_node.identity_key);
        } catch (e) {
          enqueueSnackbar(e as string, { variant: 'error', preventDuplicate: true });
        }
      })();
    } else {
      reset();
    }
  }, [shouldUpdate]);

  return {
    status,
    saturation,
    rewardEstimation,
    inclusionProbability,
  };
};
