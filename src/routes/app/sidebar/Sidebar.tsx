import React, { useEffect, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Podcast, podcastApi } from '../../../backend/podcastApi.ts'
import { ImportPodcastButton } from './ImportPodcastButton.tsx'
import { Link } from '@tanstack/react-router'
import { appHomeRoute, podcastRoute } from '../../../routeDefinitions.ts'
import styled, { keyframes } from 'styled-components'
import { WindowControls } from 'tauri-controls'
import { podcastUtil } from '../../../backend/podcastUtil.ts'
import { DownloadMonitor } from './DownloadMonitor.tsx'
import { SyncPodcastsButton } from './SyncPodcastsButton.tsx'
import { listen } from '@tauri-apps/api/event'

const SidebarWrapper = styled.div`
  overflow: auto;
  height: 100vh;
  width: 230px;
  left: 0;
  top: 0;
  bottom: 0;
  background: var(--primary-grayish);
  border-right: 1px solid var(--gray15);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  @media (prefers-color-scheme: dark) {
    background: var(--primary-grayish-dark);
  }
`

const SidebarToolbar = styled.div`
  height: 48px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  padding: 0 8px 0 0;
  margin-bottom: 4px;
  gap: 4px;
  flex-shrink: 0;
`

const SidebarLink = styled(Link)`
  display: flex;
  align-items: center;
  cursor: default;
  text-decoration: none;
  color: inherit;
  padding: 4px 12px;
  gap: 8px;
  border-radius: 0;
  margin: 0 -8px;
  min-height: 33px;

  &.active {
    background-color: var(--murrey);
    color: #FFF;
  }

  & > span {
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  & > span.material-icons-outlined {
    display: flex;
    width: 25px;
    height: 25px;
    align-items: center;
    justify-content: center;
  }
`

const PodcastImageDiv = styled.div`
  display: block;
  width: 25px;
  height: 25px;
  background-size: cover;
  position: relative;
  flex-shrink: 0;
`

const rotate = keyframes`
  to {
    transform: rotate(360deg);
  }
`

const SpinningLoader = styled.div`
  position: absolute;
  height: 100%;
  width: 100%;
  top: 0;
  left: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, .25);

  & .material-icons-outlined {
    animation: ${rotate} 1s linear infinite;
  }
`

const Divider = styled.hr`
  margin-top: 6px;
  margin-bottom: 2px;
  border-color: var(--gray25);
`

const podcastsCurrentlySyncing: Record<string, boolean> = {}

export const Sidebar: React.FC = () => {
  const query = useQuery({ queryKey: ['allPodcasts'], queryFn: podcastApi.listAll })
  const queryItems: Podcast[] = query.data ?? []
  const [syncingPodcasts, setSyncingPodcasts] = useState<Record<string, boolean>>(podcastsCurrentlySyncing)
  useEffect(() => {
    listen('sync-podcast-start', event => {
      const id = parseInt(event.payload as string).toString()
      podcastsCurrentlySyncing[id] = true
      console.log('sync-podcast-start event', podcastsCurrentlySyncing)
      setSyncingPodcasts({ ...podcastsCurrentlySyncing })
    })
  }, [setSyncingPodcasts])
  useEffect(() => {
    listen('sync-podcast-stop', event => {
      const id = parseInt(event.payload as string).toString()
      podcastsCurrentlySyncing[id] = false
      console.log('sync-podcast-stop event', podcastsCurrentlySyncing)
      setSyncingPodcasts({ ...podcastsCurrentlySyncing })
    })
  }, [setSyncingPodcasts])

  // @ts-ignore
  return (
    <SidebarWrapper>
      <SidebarToolbar
        data-tauri-drag-region={true}
      >
        <WindowControls platform="macos" style={{ marginRight: 'auto' }}/>
        <div style={{ flex: 1 }}/>
        <ImportPodcastButton/>
        <SyncPodcastsButton/>
      </SidebarToolbar>
      <div style={{ padding: 8, flex: 1, overflowY: 'auto' }}>
        <SidebarLink to={appHomeRoute.to} params={{}} search={{}} className="sidebar-link">
          <span className="material-icons-outlined">home</span>
          Home
        </SidebarLink>
        <Divider/>
        {queryItems.map(podcast => (
          // @ts-ignore
          <SidebarLink key={podcast.id} to={podcastRoute.to} search={{}} params={{ podcastId: podcast.id.toString() }}
                       className="sidebar-link">
            <PodcastImageDiv style={{
              backgroundImage: `url(${podcastUtil.imageUrl(podcast)})`,
            }}>
              {syncingPodcasts[podcast.id.toString()] && (
                <SpinningLoader>
                  <span className="material-icons-outlined">refresh</span>
                </SpinningLoader>
              )}
            </PodcastImageDiv>
            <span title={podcast.name}>{podcast.name}</span>
          </SidebarLink>
        ))}
      </div>
      <DownloadMonitor/>
    </SidebarWrapper>
  )
}
