import React, { createContext, PropsWithChildren, useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { Episode } from '../backend/podcastApi.ts'

export interface EpisodeDownloadProgressReport {
  downloadedBytes: number
  totalBytes: number
  episode: Episode
}

export type DownloadProgress = EpisodeDownloadProgressReport[]

export const DownloadContext = createContext<DownloadProgress>([])

export const DownloadContextProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const [downloadContext, setDownloadContext] = useState<DownloadProgress>([])

  useEffect(() => {
    listen<DownloadProgress>('episode-downloads', event => {
      setDownloadContext(event.payload)
    })
  }, [setDownloadContext])
  return (
    <DownloadContext.Provider value={downloadContext}>
      {children}
    </DownloadContext.Provider>
  )
}
