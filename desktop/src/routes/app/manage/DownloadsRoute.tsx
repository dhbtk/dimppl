import React, { useMemo } from 'react'
import { CoolTable, NoScrollContainer, SettingsToolbar, TableContainer } from './shared.tsx'
import { useQuery } from '@tanstack/react-query'
import { podcastApi } from '../../../backend/podcastApi.ts'
import { downloadsRoute } from '../../../routeDefinitions.ts'
import { formatBytes } from '../../../formatUtil.ts'
import { formatDate } from '../../../timeUtil.ts'
import { contextMenu } from '../../../backend/contextMenu.ts'

export const DownloadsRoute: React.FC = () => {
  const query = useQuery({
    queryKey: ['allDownloads'],
    queryFn: () => podcastApi.listAllDownloads(),
    initialData: downloadsRoute.useLoaderData()
  })
  const totalDownloadedBytes = useMemo(() => {
    return query.data?.map(it => it.fileSize)?.reduce((a, b) => a + b, 0) ?? 0
  }, [query])
  return (
    <NoScrollContainer>
      <SettingsToolbar/>
      <div style={{ margin: 8 }}>
        Espaço total usado por episódios baixados: {formatBytes(totalDownloadedBytes, 2)}</div>
      <TableContainer>
        <CoolTable>
          <thead>
          <tr>
            <th>Episódio</th>
            <th>Ouvido em</th>
            <th>Tamanho</th>
          </tr>
          </thead>
          <tbody>
          {query.data?.map(row => (
            <tr key={row.episode.id} onContextMenu={() => contextMenu.podcastEpisode(row.episode.id)}>
              <td>
                <strong>{row.episode.title}</strong>
                <br/>
                {row.podcast.name}
              </td>
              <td className="tiny">
                {row.progress.listenedSeconds !== 0 ? formatDate(row.progress.updatedAt) : '-'}
              </td>
              <td className="tiny">{formatBytes(row.fileSize, 2)}</td>
            </tr>
          ))}
          </tbody>
        </CoolTable>
      </TableContainer>
    </NoScrollContainer>
  )
}
