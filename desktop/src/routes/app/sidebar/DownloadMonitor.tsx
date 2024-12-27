import React, { useContext } from 'react'
import { DownloadContext } from '../../DownloadContextProvider.tsx'
import { formatBytes } from '../../../formatUtil.ts'

export const DownloadMonitor: React.FC = () => {
  const status = useContext(DownloadContext)
  return (
    <div style={{ height: 150 }}>
      {status.map(status => (
        <div key={status.episode.id}>
          <div style={{ whiteSpace: 'nowrap', overflow: 'hidden', fontSize: 11 }} title={status.episode.title}>
            {status.episode.title}
            <br/>
            {ratio(status.downloadedBytes, status.totalBytes)} ({formatBytes(status.downloadedBytes, 1)}/{formatBytes(status.totalBytes, 1)})
          </div>
          <div
            style={{
              width: ratio(status.downloadedBytes, status.totalBytes),
              height: 20,
              borderRadius: 4,
              backgroundColor: 'var(--murrey)',
              marginTop: 5,
              marginBottom: 10
            }}/>
        </div>
      ))}
    </div>
  )
}

function ratio (a: number, b: number): string {
  if (b === 0) {
    return '0'
  }
  return `${((a / b) * 100).toFixed(1)}%`
}
