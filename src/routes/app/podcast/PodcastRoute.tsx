import React from 'react'
import { podcastRoute } from '../../../routeDefinitions.ts'
import { Podcast } from '../../../backend/podcastApi.ts'
import { BackButton } from './BackButton.tsx'

export const PodcastRoute: React.FC = () => {
  const podcast: Podcast = podcastRoute.useLoader()
  return (
    <div>
      <BackButton/>
      {podcast.name}
    </div>
  )
}
