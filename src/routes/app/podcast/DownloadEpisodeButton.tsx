import React, { useContext } from 'react'
import { Episode, podcastApi } from '../../../backend/podcastApi.ts'
import { DownloadContext } from '../../DownloadContextProvider.tsx'
import { IconButton } from '../IconButton.tsx'

export const DownloadEpisodeButton: React.FC<{ episode: Episode }> = ({ episode }) => {
  const allDownloadsStatus = useContext(DownloadContext)
  const downloaded = episode.contentLocalPath.length !== 0
  const loading = allDownloadsStatus.find(it => it.episode.id === episode.id) !== undefined
  const onClick = () => podcastApi.downloadEpisode(episode.id)
  const icon = downloaded ? 'file_download_done' : (loading ? 'downloading' : 'download_for_offline')
  return (
    <IconButton icon={icon} disabled={loading || downloaded} onClick={onClick}/>
  )
}
