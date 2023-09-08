import React, { createContext, PropsWithChildren, useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

export interface EpisodeDownloadProgress {
  downloadedBytes: number
  totalBytes: number
}

export type DownloadProgress = Record<number, EpisodeDownloadProgress>

export const DownloadContext = createContext<DownloadProgress>({})

export const DownloadContextProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const [downloadContext, setDownloadContext] = useState<DownloadProgress>({})

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
