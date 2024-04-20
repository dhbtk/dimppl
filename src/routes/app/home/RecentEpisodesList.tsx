import React from 'react'
import { Header } from './shared.tsx'
import { appHomeRoute } from '../../../routeDefinitions.ts'
import { useQuery } from '@tanstack/react-query'
import { podcastApi } from '../../../backend/podcastApi.ts'
import { EpisodeListItem } from '../podcast/EpisodeListItem.tsx'

export const RecentEpisodesList: React.FC = () => {
  const { latest } = appHomeRoute.useLoaderData()
  const data = useQuery({ queryFn: podcastApi.listLatestEpisodes, initialData: latest, queryKey: ['allEpisodes'] })
  return (
    <>
      <Header style={{ marginBottom: -12 }}>Novos Episódios</Header>
      {data.data?.map(({ episode, podcast, progress }) => (
        <EpisodeListItem key={episode.id} episode={episode} progress={progress} podcast={podcast}/>
      ))}
    </>
  )
}
