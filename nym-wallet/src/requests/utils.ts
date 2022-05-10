import { invoke } from '@tauri-apps/api';
import { AppEnv } from '@nymproject/types';

export const getEnv = async (): Promise<AppEnv> => invoke('get_env');
