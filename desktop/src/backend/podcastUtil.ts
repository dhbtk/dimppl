import { Episode, Podcast } from './podcastApi.ts'

export const podcastUtil = {
  imageUrl: (podcast: Podcast): string => {
    if (podcast.localImagePath.length !== 0) {
      return `localimages://podcast/${podcast.id}`
    }
    return podcast.imageUrl
  },
  episodeImage: (episode: Episode, podcast: Podcast): string => {
    if (episode.imageLocalPath.length !== 0) {
      return `localimages://episode/${episode.id}`
    }
    if (episode.imageUrl.length !== 0) {
      return episode.imageUrl
    }
    return podcastUtil.imageUrl(podcast)
  }
}
