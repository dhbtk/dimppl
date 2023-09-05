import React from 'react'
import { Podcast } from '../../../backend/podcastApi.ts'
import styled from 'styled-components'
import { BackButton } from './BackButton.tsx'
import { podcastUtil } from '../../../backend/podcastUtil.ts'

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

export const PodcastDetailHeader: React.FC<{ podcast: Podcast }> = ({ podcast }) => {
  return (
    <WrapperDiv>
      <BackButton/>
      <div style={{ display: 'flex', gap: 24 }}>
        <BigImage url={podcastUtil.imageUrl(podcast)} />
        <DescriptionWrapper>
          <Title>{podcast.name}</Title>
          <Subtitle>{podcast.author}</Subtitle>
          <div dangerouslySetInnerHTML={{ __html: podcast.description }}></div>
        </DescriptionWrapper>
      </div>
    </WrapperDiv>
  )
}
