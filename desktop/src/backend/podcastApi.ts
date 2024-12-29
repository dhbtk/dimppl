import { invoke } from '@tauri-apps/api/core'

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

export interface EpisodeWithPodcast {
  episode: Episode
  podcast: Podcast
  progress: EpisodeProgress
}

export interface EpisodeWithFileSize extends EpisodeWithPodcast {
  fileSize: number
}

export interface PodcastWithStats {
  podcast: Podcast
  totalEpisodes: number
  latestEpDate: string
  lastListenedAt: string | null
}

export interface PodcastUpdateRequest {
  id: number
  url: string
}

export const podcastApi = {
  listAll: async (): Promise<Podcast[]> => {
    return await invoke<Podcast[]>('list_all_podcasts')
  },
  syncPodcasts: async (): Promise<void> => {
    return await invoke<void>('sync_podcasts')
  },
  findLastPlayed: async (): Promise<EpisodeWithPodcast | null> => {
    return await invoke<EpisodeWithPodcast | null>('find_last_played')
  },
  listListenHistory: async (): Promise<EpisodeWithPodcast[]> => {
    return await invoke<EpisodeWithPodcast[]>('list_listen_history')
  },
  listLatestEpisodes: async (): Promise<EpisodeWithPodcast[]> => {
    return await invoke<EpisodeWithPodcast[]>('list_latest_episodes')
  },
  importPodcast: async (url: string): Promise<string> => {
    return await invoke<string>('import_podcast', { url })
  },
  listEpisodes: async (podcastId: number): Promise<EpisodeWithProgress[]> => {
    return await invoke<EpisodeWithProgress[]>('list_podcast_episodes', { id: podcastId })
  },
  getEpisode: async (id: number): Promise<Episode> => {
    return await invoke<Episode>('get_episode', { id })
  },
  getEpisodeFull: async (id: number): Promise<EpisodeWithPodcast> => {
    return await invoke<EpisodeWithPodcast>('get_episode_full', { id })
  },
  downloadEpisode: async (id: number): Promise<void> => {
    return await invoke<void>('download_episode', { id })
  },
  playEpisode: async (id: number): Promise<void> => {
    return await invoke<void>('play_episode', { id })
  },
  playerAction: async (action: 'play' | 'pause' | 'skip_forwards' | 'skip_backwards'): Promise<void> => {
    return await invoke<void>('player_action', { action })
  },
  getProgressForEpisode: async (episodeId: number): Promise<EpisodeProgress> => {
    return await invoke<EpisodeProgress>('find_progress_for_episode', { episodeId })
  },
  eraseEpisodeDownload: async (id: number): Promise<void> => {
    return await invoke<void>('erase_episode_download', { id })
  },
  listAllDownloads: async (): Promise<EpisodeWithFileSize[]> => {
    return await invoke<EpisodeWithFileSize[]>('list_all_downloads')
  },
  listPodcastStats: async (): Promise<PodcastWithStats[]> => {
    return await invoke<PodcastWithStats[]>('list_podcast_stats')
  },
  updatePodcast: async (request: PodcastUpdateRequest): Promise<void> => {
    return await invoke<void>('update_podcast', { request })
  }
}
