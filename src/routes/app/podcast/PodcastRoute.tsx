import React from 'react'
import { podcastRoute } from '../../../routeDefinitions.ts'
import { Podcast } from '../../../backend/podcastApi.ts'

export const PodcastRoute: React.FC = () => {
  const podcast: Podcast = podcastRoute.useLoader()
  return (
    <div>
      {podcast.name}
    </div>
  )
}
