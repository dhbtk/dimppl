import React, { PropsWithChildren } from 'react'
import { HomeOutlined, ReloadOutlined } from '@ant-design/icons'
import { useQuery } from '@tanstack/react-query'
import { Podcast, podcastApi } from '../../backend/podcastApi.ts'
import { ImportPodcastButton } from './ImportPodcastButton.tsx'
import { Link } from '@tanstack/react-router'

const SidebarLink: React.FC<PropsWithChildren<{ to: string }>> = ({ to, children }) => {
  return (
    <Link
      to={to}
      params={{}}
      search={{}}
      style={{
        display: 'flex',
        height: '20px',

      }}
    >
      {children}
    </Link>
  )
}

export const Sidebar: React.FC = () => {
  const query = useQuery({ queryKey: ['allPodcasts'], queryFn: podcastApi.listAll })
  const queryItems: Podcast[] = query.data ?? [];

  return (
    <div style={{
      overflow: 'auto',
      height: '100vh',
      position: 'fixed',
      width: '200px',
      left: 0,
      top: 0,
      bottom: 0,
      background: '#E2E2E2' // #84C5E6
    }}>
      <div style={{
        height: 64,
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'flex-start',
        padding: '0 8px'
      }}>
        <ImportPodcastButton/>
        <button type="button" style={{ marginLeft: 'auto' }}>
          <ReloadOutlined/>
        </button>
      </div>
      <div>
        <SidebarLink to="/app">
          <HomeOutlined/>
          Home
        </SidebarLink>
        {queryItems.map(podcast => (
          <SidebarLink to={`/app/podcasts/${podcast.id}`}>
            {podcast.name}
          </SidebarLink>
        ))}
      </div>
    </div>
  )
}
