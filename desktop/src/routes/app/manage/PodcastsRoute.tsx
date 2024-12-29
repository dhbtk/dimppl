import React, { useCallback, useRef, useState } from 'react'
import { NoScrollContainer, SettingsToolbar } from './shared.tsx'
import { useQuery } from '@tanstack/react-query'
import { podcastApi, PodcastWithStats } from '../../../backend/podcastApi.ts'
import styled from 'styled-components'
import { formatDate } from '../../../timeUtil.ts'
import { PrettyButton } from '../../../components/PrettyButton.tsx'

const ListContainer = styled.div`
  padding-top: 16px;
  padding-left: 8px;
  padding-right: 8px;
  flex: 1;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: auto;
  gap: 8px;
  margin-bottom: 48px;
`

const PodcastContainer = styled.div`
  padding: 0 8px 8px;
  border-bottom: 2px solid var(--gray12);

  & > h2 {
    font-size: 110%;
    border-bottom: 1px solid var(--gray12);
  }

  & form {
    label {
      display: flex;
      gap: 4px;
      flex-direction: column;
      font-size: 90%;
      padding: 4px 0;

      input {
        padding: 2px;
      }
    }

    .button-container {
      display: flex;
      justify-content: space-between;
    }
  }
`

const PodcastStatsContainer: React.FC<{ podcastWithStats: PodcastWithStats }> = ({ podcastWithStats: item }) => {
  const detailsRef = useRef<HTMLDetailsElement>(null)
  const formRef = useRef<HTMLFormElement>(null)
  const [url, setUrl] = useState(item.podcast.feedUrl)
  const handleClose = useCallback(() => {
    if (!detailsRef.current?.open) {
      setUrl(item.podcast.feedUrl)
    }
  }, [item.podcast.feedUrl])
  const handleSubmit = useCallback(async () => {
    // TODO: actually only sync this podcast, listen for sync errors, and report to user
    const request = {
      id: item.podcast.id,
      url
    }
    return podcastApi.updatePodcast(request)
  }, [url, item])
  return (
    <PodcastContainer>
    <h2>{item.podcast.name}</h2>
    <p>{item.totalEpisodes} episódios &bull; Atualizado em {formatDate(item.latestEpDate)}</p>
    {item.lastListenedAt !== null && (<p>Ouvido em {formatDate(item.lastListenedAt)}</p>)}
    <details ref={detailsRef} onToggle={handleClose}>
      <summary>Editar</summary>
      <form onSubmit={handleSubmit} ref={formRef}>
        <label>
          <span>URL ({url})</span>
          <input type="url" value={url} onChange={e => setUrl(e.currentTarget.value)} required />
        </label>
        <div className="button-container">
          <PrettyButton type="submit" disabled={!url.trim()}>
            Salvar
          </PrettyButton>
          <PrettyButton type="button">
            Excluir
          </PrettyButton>
        </div>
      </form>
    </details>
  </PodcastContainer>
  )
}

export const PodcastsRoute: React.FC = () => {
  const query = useQuery({ queryKey: ['podcastStats'], queryFn: podcastApi.listPodcastStats })
  const queryItems: PodcastWithStats[] = query.data ?? []

  return (
    <NoScrollContainer>
      <SettingsToolbar/>
      <ListContainer>
        {queryItems.map(item => <PodcastStatsContainer key={item.podcast.id} podcastWithStats={item} />)}
      </ListContainer>
    </NoScrollContainer>
  )
}
