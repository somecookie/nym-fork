import React, { useCallback } from 'react';
import {
  Box,
  Button,
  Icon,
  IconButton,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
  Stack,
  Tooltip,
  Typography,
} from '@mui/material';
import { Delegate, Undelegate } from '../../svg-icons';
import { DelegateListItemPending } from './types';
import { MoreVertSharp } from '@mui/icons-material';

export type DelegationListItemActions = 'delegate' | 'undelegate' | 'redeem';

const BUTTON_SIZE = '32px';
const MIN_WIDTH = '150px';

export const DelegationActions: React.FC<{
  onActionClick?: (action: DelegationListItemActions) => void;
  isPending?: DelegateListItemPending;
  disableRedeemingRewards?: boolean;
}> = ({ disableRedeemingRewards, onActionClick, isPending }) => {
  if (isPending) {
    return (
      <Box py={0.5} fontSize="inherit" minWidth={MIN_WIDTH} minHeight={BUTTON_SIZE}>
        <Tooltip title="There will be a new epoch roughly every hour when your changes will take effect" arrow>
          <Typography fontSize="inherit" color="text.disabled">
            Pending {isPending.actionType === 'delegate' ? 'delegation' : 'undelegation'}...
          </Typography>
        </Tooltip>
      </Box>
    );
  }
  return (
    <Stack spacing={2} direction="row" minWidth={MIN_WIDTH}>
      <Tooltip title="Delegate more" arrow>
        <Button
          onClick={() => (onActionClick ? onActionClick('delegate') : undefined)}
          variant="contained"
          disableElevation
          sx={{ maxWidth: BUTTON_SIZE, minWidth: BUTTON_SIZE, height: BUTTON_SIZE, padding: 0 }}
        >
          <Delegate fontSize="small" />
        </Button>
      </Tooltip>
      <Tooltip title="Undelegate" arrow>
        <Button
          variant="outlined"
          sx={{ maxWidth: BUTTON_SIZE, minWidth: BUTTON_SIZE, height: BUTTON_SIZE, padding: 0 }}
          onClick={() => (onActionClick ? onActionClick('undelegate') : undefined)}
        >
          <Undelegate fontSize="small" />
        </Button>
      </Tooltip>
      <Tooltip title={disableRedeemingRewards ? 'There are no rewards to redeem' : 'Redeem rewards'} arrow>
        <span>
          <Button
            disabled={disableRedeemingRewards}
            onClick={() => (onActionClick ? onActionClick('redeem') : undefined)}
            variant="outlined"
            color="secondary"
            sx={{ maxWidth: BUTTON_SIZE, minWidth: BUTTON_SIZE, height: BUTTON_SIZE, padding: 0 }}
          >
            R
          </Button>
        </span>
      </Tooltip>
    </Stack>
  );
};

export const DelegationsActionsMenu = () => {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);
  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  const DelegationActionsMenuItem = useCallback(
    ({
      title,
      description,
      onClick,
      Icon,
    }: {
      title: string;
      description?: string;
      onClick?: () => void;
      Icon?: React.ReactNode;
    }) => (
      <MenuItem sx={{ p: 2 }} onClick={onClick}>
        <ListItemIcon sx={{ color: 'black' }}>{Icon}</ListItemIcon>
        <ListItemText sx={{ color: 'black' }} primary={title} secondary={description} />
      </MenuItem>
    ),
    [],
  );

  return (
    <>
      <IconButton onClick={handleClick}>
        <MoreVertSharp />
      </IconButton>
      <Menu anchorEl={anchorEl} open={open} onClose={handleClose}>
        <DelegationActionsMenuItem title="Delegate more" Icon={<Delegate />} onClick={handleClose} />
        <DelegationActionsMenuItem title="Undelegate" Icon={<Undelegate />} onClick={handleClose} />
        <DelegationActionsMenuItem
          title="Compound"
          description="Add your rewards to this delegation"
          Icon={<Typography sx={{ pl: 1 }}>C</Typography>}
          onClick={handleClose}
        />
        <DelegationActionsMenuItem
          title="Redeem"
          description="Trasfer your rewards to your balance"
          Icon={<Typography>R</Typography>}
          onClick={handleClose}
        />
      </Menu>
    </>
  );
};
