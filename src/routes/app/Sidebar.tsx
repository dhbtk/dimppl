import React, { useEffect, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Podcast, podcastApi } from '../../backend/podcastApi.ts'
import { ImportPodcastButton } from './ImportPodcastButton.tsx'
import { Link } from '@tanstack/react-router'
import { appHomeRoute, podcastRoute } from '../../routeDefinitions.ts'
import styled, { keyframes } from 'styled-components'
import { WindowControls } from 'tauri-controls'
import { podcastUtil } from '../../backend/podcastUtil.ts'
import { DownloadMonitor } from './DownloadMonitor.tsx'
import { SyncPodcastsButton } from './SyncPodcastsButton.tsx'
import { listen } from '@tauri-apps/api/event'

const SidebarLink = styled(Link)`
  display: flex;
  align-items: center;
  cursor: default;
  text-decoration: none;
  color: inherit;
  padding: 4px;
  gap: 8px;
  border-radius: 4px;
  min-height: 33px;

  &.active {
    background-color: #6E9BF9;
    color: #FFF;
  }

  & > span {
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }
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
    <div style={{
      overflow: 'auto',
      height: '100vh',
      width: '230px',
      left: 0,
      top: 0,
      bottom: 0,
      background: 'transparent',
      // background: '#E2E2E2', // #84C5E6
      borderRight: '1px solid #D9D9D9',
      flexShrink: 0,
      display: 'flex',
      flexDirection: 'column'
    }}>
      <div
        data-tauri-drag-region={true}
        style={{
          height: 48,
          display: 'flex',
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'flex-start',
          padding: '0 8px',
          paddingLeft: 0,
          marginBottom: 4,
          gap: 4,
          flexShrink: 0
        }}
      >
        <WindowControls platform="macos" style={{ marginRight: 'auto' }}/>
        <div style={{ flex: 1 }}/>
        <ImportPodcastButton/>
        <SyncPodcastsButton/>
      </div>
      <div style={{ padding: 8, flex: 1 }}>
        <SidebarLink to={appHomeRoute.to} params={{}} search={{}} className="sidebar-link">
          <span className="material-icons-outlined">home</span>
          Home
        </SidebarLink>
        {queryItems.map(podcast => (
          // @ts-ignore
          <SidebarLink key={podcast.id} to={podcastRoute.to} search={{}} params={{ podcastId: podcast.id.toString() }}
                       className="sidebar-link">
            <div style={{
              display: 'block',
              width: 25,
              height: 25,
              backgroundImage: `url(${podcastUtil.imageUrl(podcast)})`,
              backgroundSize: 'cover',
              position: 'relative',
              flexShrink: 0
            }}>
              {syncingPodcasts[podcast.id.toString()] && (
                <SpinningLoader>
                  <span className="material-icons-outlined">refresh</span>
                </SpinningLoader>
              )}
            </div>
            <span title={podcast.name}>{podcast.name}</span>
          </SidebarLink>
        ))}
      </div>
      <DownloadMonitor/>
    </div>
  )
}
