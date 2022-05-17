import * as React from 'react';
import type { MajorCurrencyAmount } from '@nymproject/types';

export const Currency: React.FC<{
  majorAmount?: MajorCurrencyAmount;
  showDenom?: boolean;
}> = ({ majorAmount, showDenom = true }) => {
  if (!majorAmount) {
    return <span>-</span>;
  }
  return (
    <span>
      {majorAmount.amount}
      {showDenom && ` ${majorAmount.denom}`}
    </span>
  );
};
