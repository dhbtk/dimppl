import React, { useContext } from 'react'
import { Episode, EpisodeWithPodcast, podcastApi } from '../../../backend/podcastApi.ts'
import styled from 'styled-components'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { PlayerContext } from '../../PlayerContextProvider.tsx'
import { ImageOverlayButton } from '../ImageOverlayButton.tsx'
import { Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { formatHumane } from '../../../timeUtil.ts'
import { IconButton } from '../IconButton.tsx'
import { episodeRoute } from '../../../routeDefinitions.ts'

const WrapperDiv = styled.div`
  padding: 4px 12px 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
`

const BigImage = styled.div<{url: string}>`
  width: 150px;
  height: 150px;
  border-radius: 5px;
  background-size: contain;
  background-image: url("${props => props.url}");
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
`

const DescriptionWrapper = styled.div`
  flex: 1;
  max-height: 150px;
  overflow-y: auto;
`

const Header = styled.h1`
  font-size: 24px;
`

const Title = styled(Link)`
  font-size: 20px;
  display: block;
  cursor: default;
  
  &:hover {
    text-decoration: underline;
    cursor: default;
  }
`
const Subtitle = styled(Link)`
  font-size: 16px;
  color: #8E8E8E;
  display: block;
  cursor: default;
  
  &:hover {
    text-decoration: underline;
    cursor: default;
  }
`

const PlayButton: React.FC<{ episode: Episode }> = ({ episode }) => {
  const playerStatus = useContext(PlayerContext)
  if (playerStatus.episode?.id === episode.id) {
    return (
      <ImageOverlayButton
        icon={playerStatus.isPaused ? 'play_circle' : 'pause'}
        onClick={() => podcastApi.playerAction(playerStatus.isPaused ? 'play' : 'pause')}
      />
    )
  } else {
    return (
      <ImageOverlayButton icon="play_circle" title="Play" onClick={() => podcastApi.playEpisode(episode.id)}/>
    )
  }
}

export const LastPlayedCard: React.FC<{ lastPlayed: EpisodeWithPodcast }> = ({ lastPlayed }) => {
  const query = useQuery({
    queryKey: [`episode-${lastPlayed.episode.id}`],
    queryFn: () => podcastApi.getEpisodeFull(lastPlayed.episode.id),
    initialData: lastPlayed
  })
  const { episode, progress, podcast } = query.data
  return (
    <WrapperDiv>
      <Header>Continue Ouvindo</Header>
      <div style={{ display: 'flex', gap: 24 }}>
        <BigImage url={podcastUtil.imageUrl(podcast)}>
          <PlayButton episode={episode}/>
        </BigImage>
        <DescriptionWrapper>
          <Title to={episodeRoute.to} search={{}} params={{ episodeId: episode.id.toString() }}>{episode.title}</Title>
          <Subtitle to={episodeRoute.to} search={{}} params={{ episodeId: episode.id.toString() }}>{podcast.name}</Subtitle>
          <div style={{ display: 'flex', gap: 16 }}>
            <div style={{ flex: 1, display: 'flex', alignItems: 'center' }}>
              {
                progress.listenedSeconds === 0 ? (
                  formatHumane(episode.length)
                ) : (
                  `${formatHumane(episode.length - progress.listenedSeconds)} restantes`
                )
              }
            </div>
            <IconButton icon="more_vert"/>
          </div>
        </DescriptionWrapper>
      </div>
    </WrapperDiv>
  )
}
