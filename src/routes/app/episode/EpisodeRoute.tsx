import React from 'react'
import { episodeRoute } from '../../../routeDefinitions.ts'
import styled from 'styled-components'
import { BackButton } from '../podcast/BackButton.tsx'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { episodeDate, formatHumane } from '../../../timeUtil.ts'
import { DownloadEpisodeButton } from '../podcast/DownloadEpisodeButton.tsx'
import { PlayButton } from '../podcast/PlayButton.tsx'
import { useQuery } from '@tanstack/react-query'
import { podcastApi } from '../../../backend/podcastApi.ts'
import { IconButton } from '../IconButton.tsx'

const WrapperDiv = styled.div`
  padding: 4px 12px 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
`

const BigImage = styled.div<{url: string}>`
  width: 200px;
  height: 200px;
  border-radius: 5px;
  background-size: contain;
  background-image: url("${props => props.url}");
  flex-shrink: 0;
`

const DescriptionWrapper = styled.div`
  flex: 1;
  max-height: 200px;
  overflow-y: auto;
`

const Title = styled.h1`
  font-size: 24px;
`
const Subtitle = styled.h2`
  font-size: 20px;
  color: #8E8E8E;
`

const HorizontalInfoList = styled.div`
  display: flex;
  gap: 8px;
`

const InfoContainer = styled.div`
  min-width: 125px;
  max-width: 125px;
  width: 125px;
  flex-shrink: 0;
  font-size: 14px;
`

const InfoHeader = styled.div`
  display: block;
  font-weight: bold;
`

const InfoData = styled.div`
  
`

export const EpisodeRoute: React.FC = () => {
  const initialData = episodeRoute.useLoader()
  const query = useQuery({
    queryKey: [`episode-${initialData.episode.id}`],
    queryFn: () => podcastApi.getEpisodeFull(initialData.episode.id),
    initialData: initialData
  })
  const { episode, progress, podcast } = query.data

  return (
    <div style={{ flex: '1', display: 'flex', flexDirection: 'column', gap: 8, maxHeight: '100vh', overflow: 'auto' }}>
      <WrapperDiv>
        <BackButton/>
        <div style={{ display: 'flex', gap: 24 }}>
          <BigImage url={podcastUtil.imageUrl(podcast)}/>
          <DescriptionWrapper>
            <Title>{episode.title}</Title>
            <Subtitle>{podcast.name}</Subtitle>
          </DescriptionWrapper>
        </div>
        <HorizontalInfoList>
          <InfoContainer>
            <InfoHeader>Publicado em</InfoHeader>
            <InfoData>{episodeDate(episode.episodeDate)}</InfoData>
          </InfoContainer>
          <InfoContainer>
            <InfoHeader>Duração</InfoHeader>
            <InfoData>{formatHumane(episode.length)}</InfoData>
          </InfoContainer>
        </HorizontalInfoList>
        <div style={{ display: 'flex', gap: 16 }}>
          {episode.contentLocalPath.length === 0 ? (
            <DownloadEpisodeButton episode={episode}/>
          ) : (
            <PlayButton episode={episode}/>
          )}
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
      </WrapperDiv>
      <hr style={{ margin: 8 }}/>
      <div style={{ padding: 8 }} dangerouslySetInnerHTML={{ __html: episode.description }}></div>
    </div>
  )
}
