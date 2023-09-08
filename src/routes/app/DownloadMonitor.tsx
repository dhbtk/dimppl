import React, { useContext } from 'react'
import { DownloadContext } from '../DownloadContextProvider.tsx'

export const DownloadMonitor: React.FC = () => {
  const status = useContext(DownloadContext)
  return (
    <div style={{ height: 150 }}>
      {JSON.stringify(status)}
    </div>
  )
}
