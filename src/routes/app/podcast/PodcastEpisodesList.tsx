import React from 'react'
import { Podcast, podcastApi } from '../../../backend/podcastApi.ts'
import { useQuery } from '@tanstack/react-query'
import { EpisodeListItem } from './EpisodeListItem.tsx'

export const PodcastEpisodesList: React.FC<{ podcast: Podcast }> = ({ podcast }) => {
  const query = useQuery({
    queryKey: [`podcastEpisodes-${podcast.id}`],
    queryFn: () => podcastApi.listEpisodes(podcast.id),
    initialData: []
  })
  return (
    <div style={{ flex: '1', overflow: 'auto' }}>
      {query.data.map(({ episode, progress }) => (
        <EpisodeListItem episode={episode} podcast={podcast} key={episode.id} progress={progress}/>
      ))}
    </div>
  )
}
