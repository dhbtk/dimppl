import React from 'react'
import styled from 'styled-components'
import { useQuery } from '@tanstack/react-query'
import { EpisodeWithPodcast, podcastApi } from '../../../backend/podcastApi.ts'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { Link } from '@tanstack/react-router'
import { formatHumane } from '../../../timeUtil.ts'
import { Header } from './shared.tsx'

const HorizontalTileList = styled.div`
  display: flex;
  overflow-x: auto;
  width: calc(100vw - 230px);
  align-items: flex-start;
  padding: 0 12px;
  gap: 12px;
`

const TileWrapper = styled.div`
  display: flex;
  flex-direction: column;
  width: 125px;
  max-width: 125px;
  flex-shrink: 0;
`

const BigImage = styled.div<{ url: string }>`
  width: 125px;
  height: 125px;
  border-radius: 5px;
  background-size: contain;
  background-image: url("${props => props.url}");
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
`

const TileLink = styled(Link)`
  cursor: default;
  font-weight: bold;
  font-size: 11px;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  overflow: hidden;

  &:hover {
    cursor: default;
    text-decoration: underline;
  }
`

const DateDisplay = styled.div`
  font-size: 11px;
  line-height: 1.4;
`

const EpisodeTile: React.FC<{ data: EpisodeWithPodcast }> = ({ data }) => {
  const { episode, podcast, progress } = data
  return (
    <TileWrapper>
      <BigImage url={podcastUtil.imageUrl(podcast)}/>
      <TileLink title={episode.title} to={`episode/${episode.id}`} search={{}} params={{}}>{episode.title}</TileLink>
      <DateDisplay>
        {
          progress.completed ? 'Reproduzido' : (
            progress.listenedSeconds === 0 ? (
              formatHumane(episode.length)
            ) : (
              `${formatHumane(episode.length - progress.listenedSeconds)} restantes`
            ))
        }
      </DateDisplay>
    </TileWrapper>
  )
}

export const ListenHistoryTileList: React.FC = () => {
  const query = useQuery({ queryFn: podcastApi.listListenHistory, initialData: [] })
  const queryItems = query.data
  if (queryItems.length === 0) {
    return <></>
  }
  return (
    <div style={{ display: 'flex', flexDirection: 'column' }}>
      <Header>Reproduzidos Recentemente</Header>
      <HorizontalTileList>
        {queryItems.map(data => (
          <EpisodeTile key={data.episode.id} data={data}/>
        ))}
      </HorizontalTileList>
    </div>
  )
}
