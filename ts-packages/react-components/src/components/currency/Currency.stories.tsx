import * as React from 'react';
import { ComponentMeta } from '@storybook/react';
import { Currency } from './Currency';

export default {
  title: 'Currency/Currency display',
  component: Currency,
} as ComponentMeta<typeof Currency>;

export const Mainnet = () => <Currency majorAmount={{ amount: '42.123456', denom: 'NYM' }} />;

export const Testnet = () => <Currency majorAmount={{ amount: '42.123456', denom: 'NYMT' }} />;

export const Empty = () => <Currency />;
