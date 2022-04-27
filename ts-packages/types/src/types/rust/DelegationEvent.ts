import type { DelegationResult } from './DelegationResult';
import type { PendingUndelegate } from './PendingUndelegate';

export type DelegationEvent = { Delegate: DelegationResult } | { Undelegate: PendingUndelegate };
