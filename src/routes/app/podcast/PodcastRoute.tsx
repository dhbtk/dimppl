import React from 'react'
import { podcastRoute } from '../../../routeDefinitions.ts'
import { Podcast } from '../../../backend/podcastApi.ts'
import { PodcastDetailHeader } from './PodcastDetailHeader.tsx'

export const PodcastRoute: React.FC = () => {
  const podcast: Podcast = podcastRoute.useLoader()
  return (
    <div style={{ flex: '1', display: 'flex', flexDirection: 'column', gap: 8 }}>
      <PodcastDetailHeader podcast={podcast}/>
      <hr style={{ margin: 8 }}/>
    </div>
  )
}
