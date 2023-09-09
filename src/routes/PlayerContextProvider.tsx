import { Episode, Podcast } from '../backend/podcastApi.ts'
import React, { createContext, PropsWithChildren, useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

export interface PlayerStatus {
  isPaused: boolean
  episode?: Episode
  podcast?: Podcast
  elapsed: number
}

const defaultState: PlayerStatus = {
  isPaused: true,
  elapsed: 0
}

export const PlayerContext = createContext(defaultState)

export const PlayerContextProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const [playerContext, setPlayerContext] = useState(defaultState)
  useEffect(() => {
    listen<PlayerStatus>('player-status', event => {
      setPlayerContext(event.payload)
    })
  }, [setPlayerContext])
  return (
    <PlayerContext.Provider value={playerContext}>
      {children}
    </PlayerContext.Provider>
  )
}
