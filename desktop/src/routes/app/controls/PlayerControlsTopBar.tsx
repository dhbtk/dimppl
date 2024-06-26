import React, { useContext } from 'react'
import styled from 'styled-components'
import { PlayerContext } from '../../PlayerContextProvider.tsx'
import { ToolbarIconButton } from './ToolbarIconButton.tsx'
import { podcastApi } from '../../../backend/podcastApi.ts'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { VolumeSlider } from './VolumeSlider.tsx'
import { episodeDate, formatHms, ratio } from '../../../timeUtil.ts'

const TopBar = styled.div`
  height: 48px;
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 0 8px;
  border-bottom: 1px solid var(--gray07);
  flex-shrink: 0;
`

const ContentAligner = styled.div`
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
`

const DisplayIsland = styled.div`
  height: 40px;
  width: 50%;
  flex-shrink: 0;
  border: 1px solid var(--gray07);
  border-radius: 4px;
  display: flex;
`

const ImageBox = styled.div<{ url: string }>`
  width: 40px;
  height: 40px;
  background-size: contain;
  background-image: url("${props => props.url}");
  flex-shrink: 0;
`

const RightSide = styled.div`
  height: 40px;
  width: 0;
  display: flex;
  flex: 1;
  flex-direction: column;
`

const TextBox = styled.div`
  font-size: 10px;
  text-align: center;
  flex: 1;
  padding-top: 3px;
  position: relative;

  & p {
    margin: 0 5px;
    line-height: 1.35;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  & p.left {
    position: absolute;
    left: -3px;
    bottom: 0;
    color: var(--gray50);
    font-size: 9px;
  }

  & p.right {
    position: absolute;
    right: -3px;
    bottom: 0;
    color: var(--gray50);
    font-size: 9px;
  }
`

const ProgressBarContainer = styled.div`
  height: 3px;
  flex-shrink: 0;
  background-color: var(--gray12);
`

const ProgressBar = styled.div<{ percent: string }>`
  height: 3px;
  width: ${props => props.percent};
  background-color: var(--gray05);
`

export const PlayerControlsTopBar: React.FC = () => {
  const playerStatus = useContext(PlayerContext)
  return (
    <TopBar data-tauri-drag-region={true}>
      <ContentAligner data-tauri-drag-region={true}>
        <ToolbarIconButton icon="fast_rewind" disabled={playerStatus.episode === undefined}
                           onClick={() => podcastApi.playerAction('skip_backwards')}/>
        <ToolbarIconButton icon={playerStatus.isPaused ? 'play_arrow' : 'pause'}
                           disabled={playerStatus.episode === undefined}
                           onClick={() => podcastApi.playerAction(playerStatus.isPaused ? 'play' : 'pause')}/>
        <ToolbarIconButton icon="fast_forward" disabled={playerStatus.episode === undefined}
                           onClick={() => podcastApi.playerAction('skip_forwards')}/>
      </ContentAligner>
      {(playerStatus.episode === null || playerStatus.episode === undefined) ? <DisplayIsland/> : (
        <DisplayIsland>
          <ImageBox url={playerStatus.episode.imageUrl || podcastUtil.imageUrl(playerStatus.podcast!)}/>
          <RightSide>
            <TextBox>
              <p
                title={playerStatus.episode.title}>{playerStatus.loading ? 'Carregando...' : playerStatus.episode.title}</p>
              <p style={{ color: 'var(--gray50)' }}>
                {playerStatus.podcast?.name}
                {' — '}
                {episodeDate(playerStatus.episode.episodeDate)}
              </p>
              <p className="left">{formatHms(playerStatus.elapsed)}</p>
              <p className="right">{formatHms(playerStatus.duration)}</p>
            </TextBox>
            <ProgressBarContainer>
              <ProgressBar percent={ratio(playerStatus.elapsed, playerStatus.duration)}/>
            </ProgressBarContainer>
          </RightSide>
        </DisplayIsland>
      )}
      <ContentAligner data-tauri-drag-region={true}>
        <VolumeSlider/>
      </ContentAligner>
    </TopBar>
  )
}
