import { ReactJSXElement } from '@emotion/react/types/jsx-namespace';
import { Card, CardHeader, CardContent, Typography } from '@mui/material';
import React, { ReactEventHandler } from 'react';


type ContentCardProps = {
  title?: string | ReactJSXElement,
  subtitle?: string,
  Icon?: React.ReactNode,
  Action?: React.ReactNode,
  errorMsg?: string,
  onClick?: ReactEventHandler,
}

export const ContentCard: React.FC<ContentCardProps> = ({
  title,
  Icon,
  Action,
  subtitle,
  errorMsg,
  children,
  onClick,
}) => (
  <Card onClick={onClick}>
    <CardHeader
      title={title}
      avatar={Icon}
      action={Action}
      subheader={subtitle}
    />
    {children && <CardContent>{children}</CardContent>}
    {errorMsg && (
      <Typography
        variant="body2"
        sx={{ color: 'danger', padding: 2 }}
      >
        {errorMsg}
      </Typography>
    )}
  </Card>
);