import { invoke } from '@tauri-apps/api'

export interface Config {
  userAccessKey: string
  deviceName: string
  accessToken: string
  volume: number
  playbackSpeed: number
}

export const configApi = {
  load: async (): Promise<Config> => {
    return invoke<Config>('get_config')
  },
  save: async (newConfig: Config): Promise<Config> => {
    await invoke<void>('set_config', { newConfig })
    return newConfig
  },
  registerNewUser: async (): Promise<Config> => {
    await invoke<void>('register_user')
    return await configApi.load()
  },
  registerDevice: async (deviceName: string): Promise<Config> => {
    await invoke<void>('register_device', { deviceName })
    return await configApi.load()
  },
  setVolume: async (volume: number): Promise<void> => {
    await invoke<void>('set_volume', { volume })
  }
}
