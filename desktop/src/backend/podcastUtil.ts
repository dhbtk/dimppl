import { Podcast } from './podcastApi.ts'

export const podcastUtil = {
  imageUrl: (podcast: Podcast): string => {
    if (podcast.localImagePath.length !== 0) {
      return `localimages://podcast/${podcast.id}`
    }
    return podcast.imageUrl
  }
}
