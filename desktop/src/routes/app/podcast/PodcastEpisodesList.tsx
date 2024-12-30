import React from 'react'
import { Podcast, podcastApi } from '../../../backend/podcastApi.ts'
import { useQuery } from '@tanstack/react-query'
import { EpisodeListItem } from './EpisodeListItem.tsx'
import { Virtuoso } from 'react-virtuoso'

export const PodcastEpisodesList: React.FC<{ podcast: Podcast }> = ({ podcast }) => {
  const query = useQuery({
    queryKey: [`podcastEpisodes-${podcast.id}`],
    queryFn: () => podcastApi.listEpisodes(podcast.id),
    initialData: []
  })
  return (
    <div style={{ flex: '1', overflow: 'auto', position: 'relative' }}>
      <Virtuoso
        totalCount={query.data.length}
        itemContent={(index) => (
          <EpisodeListItem
            episode={query.data[index].episode}
            podcast={podcast}
            progress={query.data[index].progress}
            showPodcastName={false}
          />
        )}
      />
    </div>
  )
}
