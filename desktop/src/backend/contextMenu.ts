import { invoke } from '@tauri-apps/api/core'

export const contextMenu = {
  podcastEpisode: async (id: number) => {
    return invoke<void>('show_context_menu', {
      menuOption: {
        PodcastEpisode: { id }
      }
    })
  }
}
