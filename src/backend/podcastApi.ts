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

export interface EpisodeWithProgress {
  episode: Episode
  progress: EpisodeProgress
}

export interface Episode {
  id: number
  guid: string
  podcastId: number
  contentLocalPath: string
  contentUrl: string
  description: string
  imageLocalPath: string
  imageUrl: string
  length: number
  link: string
  episodeDate: string
  title: string
}

export interface EpisodeProgress {
  id: number
  episodeId: number
  completed: boolean
  listenedSeconds: number
  updatedAt: string
}

export const podcastApi = {
  listAll: async (): Promise<Podcast[]> => {
    return await invoke<Podcast[]>('list_all_podcasts')
  },
  importPodcast: async (url: string): Promise<void> => {
    await invoke<void>('import_podcast', { url })
  },
  listEpisodes: async (podcastId: number): Promise<EpisodeWithProgress[]> => {
    return await invoke<EpisodeWithProgress[]>('list_podcast_episodes', { id: podcastId })
  }
}
