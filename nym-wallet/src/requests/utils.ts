import { invoke } from '@tauri-apps/api';
import { AppEnv } from 'src/types';

export const getEnv = async (): Promise<AppEnv> => invoke('get_env');
