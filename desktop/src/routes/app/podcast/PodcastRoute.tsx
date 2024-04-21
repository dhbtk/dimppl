import React from 'react'
import { podcastRoute } from '../../../routeDefinitions.ts'
import { Podcast } from '../../../backend/podcastApi.ts'
import { PodcastDetailHeader } from './PodcastDetailHeader.tsx'
import { PodcastEpisodesList } from './PodcastEpisodesList.tsx'

export const PodcastRoute: React.FC = () => {
  const podcast: Podcast = podcastRoute.useLoaderData()
  return (
    <div style={{ flex: '1', display: 'flex', flexDirection: 'column', gap: 8, maxHeight: '100vh' }}>
      <PodcastDetailHeader podcast={podcast}/>
      <hr style={{ margin: 8 }}/>
      <PodcastEpisodesList podcast={podcast}/>
    </div>
  )
}
