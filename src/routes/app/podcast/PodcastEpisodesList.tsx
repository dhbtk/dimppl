import React from 'react'
import { Podcast, podcastApi } from '../../../backend/podcastApi.ts'
import { useQuery } from '@tanstack/react-query'
import styled from 'styled-components'
import { Link } from '@tanstack/react-router'
import { podcastUtil } from '../../../backend/podcastUtil.ts'

const EpisodeWrapper = styled.div`
  margin: 8px;
  padding: 8px;
  display: flex;
  gap: 8px;
`

const EpisodeImageBox = styled.div<{url: string}>`
  width: 150px;
  height: 150px;
  border-radius: 5px;
  background-size: contain;
  background-image: url("${props => props.url}");
  flex-shrink: 0;
`

const EpisodeInfoBox = styled.div`
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 0;
`

const EpisodeLink = styled(Link)`
  font-weight: bold;
  text-decoration: none;
  display: inline-block;
  height: 20px;
  flex-shrink: 0;
  cursor: default;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  
  &:hover {
    text-decoration: underline;
    cursor: default;
  }
`

const EpisodeDescription = styled.div`
  height: 120px;
  display: -webkit-box;
  font-size: 11px;
  line-height: 1.4;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 8;
  overflow: hidden;
  text-overflow: ellipsis;
`

export const PodcastEpisodesList: React.FC<{ podcast: Podcast }> = ({ podcast }) => {
  const query = useQuery({
    queryKey: [`podcastEpisodes-${podcast.id}`],
    queryFn: () => podcastApi.listEpisodes(podcast.id),
    initialData: []
  })
  return (
    <div style={{ flex: '1', overflow: 'auto' }}>
      {query.data.map(({ episode }) => (
        <EpisodeWrapper key={episode.id}>
          <EpisodeImageBox url={episode.imageUrl || podcastUtil.imageUrl(podcast)}/>
          <EpisodeInfoBox>
            <EpisodeLink to="/app" search={{}} params={{}} title={episode.title}>
              {episode.title}
            </EpisodeLink>
            <EpisodeDescription dangerouslySetInnerHTML={{ __html: episode.description }}/>
          </EpisodeInfoBox>
        </EpisodeWrapper>
      ))}
    </div>
  )
}
