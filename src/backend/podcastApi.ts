import { invoke } from '@tauri-apps/api'

export interface Podcast {
  id: number
  guid: string
  author: string
  localImagePath: string
  imageUrl: string
  feedUrl: string
  name: string
  description: string
  createdAt: string
  updatedAt: string
}

export const podcastApi = {
  listAll: async (): Promise<Podcast[]> => {
    return await invoke<Podcast[]>('list_all_podcasts')
  },
  importPodcast: async (url: string): Promise<void> => {
    await invoke<void>('import_podcast', { url })
  }
}
