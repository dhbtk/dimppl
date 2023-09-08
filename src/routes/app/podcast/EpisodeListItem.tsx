import React, { useContext } from 'react'
import { Episode, Podcast, podcastApi } from '../../../backend/podcastApi.ts'
import { useQuery } from '@tanstack/react-query'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { IconButton } from '../IconButton.tsx'
import styled from 'styled-components'
import { Link } from '@tanstack/react-router'
import { DownloadContext } from '../../DownloadContextProvider.tsx'

const EpisodeWrapper = styled.div`
  margin: 8px;
  padding: 8px;
  display: flex;
  gap: 8px;
`

const EpisodeImageBox = styled.div<{ url: string }>`
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
  flex: 1;
  height: 120px;
  display: -webkit-box;
  font-size: 11px;
  line-height: 1.4;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 8;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-right: 10px;
`

const EpisodeControls = styled.div`
  border-left: 1px solid #E6e6e6;
  padding-left: 10px;
  width: 120px;
  height: 120px;
  flex-shrink: 0;
`

const DateDisplay = styled.div`
  font-size: 11px;
  line-height: 1.4;
`

const DownloadEpisodeButton: React.FC<{ episode: Episode }> = ({ episode }) => {
  const allDownloadsStatus = useContext(DownloadContext)
  const downloaded = episode.contentLocalPath.length !== 0
  const loading = allDownloadsStatus[episode.id] !== undefined
  const onClick = () => podcastApi.downloadEpisode(episode.id)
  const icon = downloaded ? 'file_download_done' : (loading ? 'downloading' : 'download_for_offline')
  return (
    <IconButton icon={icon} disabled={loading || downloaded} onClick={onClick}/>
  )
}

export const EpisodeListItem: React.FC<{ episode: Episode, podcast: Podcast }> = ({ episode: initialEpisode, podcast }) => {
  const query = useQuery({
    queryKey: [`episode-${initialEpisode.id}`],
    queryFn: () => podcastApi.getEpisode(initialEpisode.id),
    initialData: initialEpisode
  })
  const episode: Episode = query.data
  return (
    <EpisodeWrapper key={episode.id}>
      <EpisodeImageBox url={episode.imageUrl || podcastUtil.imageUrl(podcast)}/>
      <EpisodeInfoBox>
        <EpisodeLink to="/app" search={{}} params={{}} title={episode.title}>
          {episode.title}
        </EpisodeLink>
        <div style={{ display: 'flex', height: 120 }}>
          <EpisodeDescription dangerouslySetInnerHTML={{ __html: episode.description }}/>
          <EpisodeControls>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <DateDisplay>{episodeDate(episode.episodeDate)}</DateDisplay>
              <IconButton icon="play_circle" title="Play"/>
            </div>
            <div title={episode.contentUrl} style={{ whiteSpace: 'nowrap', overflow: 'hidden' }}>{episode.contentUrl}</div>
            <div title={episode.contentLocalPath} style={{ whiteSpace: 'nowrap', overflow: 'hidden' }}>{episode.contentLocalPath}</div>
            <DownloadEpisodeButton episode={episode}/>
          </EpisodeControls>
        </div>
      </EpisodeInfoBox>
    </EpisodeWrapper>
  )
}

const formatter = new Intl.DateTimeFormat()

function episodeDate(dateStr: string): string {
  const date = new Date(dateStr)
  return formatter.format(date)
}
