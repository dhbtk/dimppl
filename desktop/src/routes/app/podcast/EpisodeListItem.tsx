import React from 'react'
import { Episode, EpisodeProgress, EpisodeWithPodcast, Podcast, podcastApi } from '../../../backend/podcastApi.ts'
import { useQuery } from '@tanstack/react-query'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { IconButton } from '../IconButton.tsx'
import styled from 'styled-components'
import { Link } from '@tanstack/react-router'
import { formatDate, formatHumane, ratio } from '../../../timeUtil.ts'
import { PlayButton } from './PlayButton.tsx'
import { DownloadEpisodeButton } from './DownloadEpisodeButton.tsx'
import { episodeRoute, podcastRoute } from '../../../routeDefinitions.ts'
import { contextMenu } from '../../../backend/contextMenu.ts'

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

  &.notbold {
    font-weight: normal;
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

  &.small {
    height: 100px;
    -webkit-line-clamp: 7;
    line-height: 1.3;
  }
`

const EpisodeControls = styled.div`
  border-left: 1px solid #E6e6e6;
  padding-left: 10px;
  width: 150px;
  height: 100%;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
`

const DateDisplay = styled.div`
  font-size: 11px;
  line-height: 1.4;
`

const ProgressBarContainer = styled.div`
  height: 3px;
  flex-shrink: 0;
  background-color: #E0E0E0;
`

const ProgressBar = styled.div<{ percent: string }>`
  height: 3px;
  width: ${props => props.percent};
  background-color: #808080;
`

export const EpisodeListItem: React.FC<{
  episode: Episode,
  podcast: Podcast,
  progress: EpisodeProgress,
  style?: React.CSSProperties,
  showPodcastName: boolean
}> = ({ episode: initialEpisode, podcast, progress, style, showPodcastName }) => {
  const query = useQuery({
    queryKey: [`episode-${initialEpisode.id}`],
    queryFn: () => podcastApi.getEpisodeFull(initialEpisode.id),
    initialData: { episode: initialEpisode, podcast, progress } as EpisodeWithPodcast
  })
  const { episode } = query.data
  const myStyles = { ...style, width: 'calc(100% - 8px)' }
  return (
    <EpisodeWrapper key={episode.id} style={myStyles} onContextMenu={(e) => {
      contextMenu.podcastEpisode(episode.id)
      e.preventDefault()
    }}>
      <EpisodeImageBox url={podcastUtil.episodeImage(episode, podcast)}/>
      <EpisodeInfoBox>
        <EpisodeLink to={episodeRoute.to} search={{}} params={{ episodeId: episode.id.toString() }}
                     title={episode.title}>
          {episode.title}
        </EpisodeLink>
        {showPodcastName && (
          <EpisodeLink className="notbold" to={podcastRoute.to} params={{ podcastId: podcast.id.toString() }}
                       title={podcast.name}>
            {podcast.name}
          </EpisodeLink>
        )}
        <div style={{ marginTop: 10, display: 'flex', height: showPodcastName ? 100 : 120 }}>
          <EpisodeDescription className={showPodcastName ? 'small' : ''}
                              dangerouslySetInnerHTML={{ __html: episode.description }}/>
          <EpisodeControls>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <DateDisplay>{formatDate(episode.episodeDate)}</DateDisplay>
              {episode.contentLocalPath.length === 0 ? (
                <DownloadEpisodeButton episode={episode}/>
              ) : (
                <PlayButton episode={episode}/>
              )}
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <ProgressDisplay episode={episode} progress={progress}/>
              <IconButton icon="more_vert" onClick={() => contextMenu.podcastEpisode(episode.id)}/>
            </div>
          </EpisodeControls>
        </div>
      </EpisodeInfoBox>
    </EpisodeWrapper>
  )
}

const ProgressDisplay: React.FC<{ episode: Episode, progress: EpisodeProgress }> = ({
  episode,
  progress: initialProgress
}) => {
  const query = useQuery({
    queryKey: [`episodeProgress-${initialProgress.id}`],
    queryFn: () => podcastApi.getProgressForEpisode(episode.id),
    initialData: initialProgress
  })
  const progress = query.data
  if (progress.completed) {
    return (
      <div style={{ display: 'flex', alignItems: 'center', gap: 4, flex: 1 }}>
        <span>Reproduzido</span>
        <IconButton icon="check"/>
      </div>
    )
  }
  return (
    <div style={{ display: 'flex', flexDirection: 'column', flex: 1, gap: 4 }}>
      <DateDisplay>
        {
          progress.listenedSeconds === 0 ? (
            formatHumane(episode.length)
          ) : (
            `${formatHumane(episode.length - progress.listenedSeconds)} restantes`
          )
        }
      </DateDisplay>
      <ProgressBarContainer style={{ visibility: progress.listenedSeconds === 0 ? 'hidden' : 'visible' }}>
        <ProgressBar percent={ratio(progress.listenedSeconds, episode.length)}/>
      </ProgressBarContainer>
    </div>
  )
}
