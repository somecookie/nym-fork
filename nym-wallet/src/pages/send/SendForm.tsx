import React, { useContext } from 'react';
import { useFormContext } from 'react-hook-form';
import { Grid, TextField } from '@mui/material';
import { CurrencyFormField } from '@nymproject/react/currency/CurrencyFormField';
import { Fee } from '../../components';
import { ClientContext } from 'src/context/main';

export const SendForm = () => {
  const {
    register,
    setValue,
    formState: { errors },
  } = useFormContext();

  const { clientDetails } = useContext(ClientContext);

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <TextField
          {...register('to')}
          required
          variant="outlined"
          id="to"
          name="to"
          label="To"
          fullWidth
          autoFocus
          error={!!errors.to}
          helperText={errors.to?.message}
        />
      </Grid>
      <Grid item xs={12}>
        <CurrencyFormField
          required
          fullWidth
          placeholder="Amount"
          onChanged={(val) => setValue('amount', val.amount)}
          validationError={errors.amount?.message}
          denom={clientDetails?.denom}
        />
      </Grid>
      <Grid item xs={12}>
        <Fee feeType="Send" />
      </Grid>
    </Grid>
  );
};
