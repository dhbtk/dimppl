import React from 'react'
import { useQuery } from '@tanstack/react-query'
import { Podcast, podcastApi } from '../../backend/podcastApi.ts'
import { ImportPodcastButton } from './ImportPodcastButton.tsx'
import { Link } from '@tanstack/react-router'
import { appHomeRoute, podcastRoute } from '../../routeDefinitions.ts'
import styled from 'styled-components'
import { WindowControls } from 'tauri-controls'
import { ToolbarButton } from './ToolbarButton.tsx'
import { podcastUtil } from '../../backend/podcastUtil.ts'
import { DownloadMonitor } from './DownloadMonitor.tsx'

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

export const Sidebar: React.FC = () => {
  const query = useQuery({ queryKey: ['allPodcasts'], queryFn: podcastApi.listAll })
  const queryItems: Podcast[] = query.data ?? []

  return (
    <div style={{
      overflow: 'auto',
      height: 'calc(100vh - 2px)',
      width: '230px',
      left: 0,
      top: 0,
      bottom: 0,
      background: '#E2E2E2', // #84C5E6
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
        <ToolbarButton type="button">
          <span className="material-icons-outlined">refresh</span>
        </ToolbarButton>
      </div>
      <div style={{ padding: 8, flex: 1 }}>
        <SidebarLink to={appHomeRoute.to} params={{}} search={{}} className="sidebar-link">
          <span className="material-icons-outlined">home</span>
          Home
        </SidebarLink>
        {queryItems.map(podcast => (
          <SidebarLink key={podcast.id} to={podcastRoute.to} search={{}} params={{ podcastId: podcast.id.toString() }} className="sidebar-link">
            <img src={podcastUtil.imageUrl(podcast)} width={25} height={25} alt=""/>
            <span title={podcast.name}>{podcast.name}</span>
          </SidebarLink>
        ))}
      </div>
      <DownloadMonitor/>
    </div>
  )
}
