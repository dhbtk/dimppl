import React, { useContext } from 'react'
import { Episode, podcastApi } from '../../../backend/podcastApi.ts'
import { PlayerContext } from '../../PlayerContextProvider.tsx'
import { IconButton } from '../IconButton.tsx'

export const PlayButton: React.FC<{ episode: Episode }> = ({ episode }) => {
  const playerStatus = useContext(PlayerContext)
  if (playerStatus.episode?.id === episode.id) {
    return (
      <IconButton
        icon={playerStatus.isPaused ? 'play_circle' : 'pause'}
        onClick={() => podcastApi.playerAction(playerStatus.isPaused ? 'play' : 'pause')}
      />
    )
  } else {
    return (
      <IconButton icon="play_circle" title="Play" onClick={() => podcastApi.playEpisode(episode.id)}/>
    )
  }
}
