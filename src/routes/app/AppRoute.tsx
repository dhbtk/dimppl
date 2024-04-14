import React, { useEffect } from 'react'
import { Outlet, useNavigate } from '@tanstack/react-router'
import { Sidebar } from './sidebar/Sidebar.tsx'
import { PlayerControlsTopBar } from './controls/PlayerControlsTopBar.tsx'
import {
  appHomeRoute,
  downloadsRoute,
  episodeRoute,
  podcastRoute,
  podcastsRoute,
  settingsRoute
} from '../../routeDefinitions.ts'
import { RootDiv } from '../../components/RootDiv.tsx'
import { listen } from '@tauri-apps/api/event'

export interface NavigationEvent {
  type: 'Home' | 'Podcast' | 'Episode' | 'Settings' | 'Podcasts' | 'Downloads',
  id?: number
}

export const AppRoute: React.FC = () => {
  const navigate = useNavigate()
  useEffect(() => {
    document.addEventListener('keydown', e => {
      if ((e.ctrlKey || e.metaKey) && e.key === ',') {
        navigate({ to: settingsRoute.to })
      }
    })
    listen<NavigationEvent>('do-navigation', ev => {
      switch (ev.payload.type) {
        case 'Home':
          navigate({ to: appHomeRoute.to })
          break
        case 'Podcast':
          navigate({ to: podcastRoute.to, params: { podcastId: ev.payload.id!.toString() } })
          break
        case 'Episode':
          navigate({ to: episodeRoute.to, params: { episodeId: ev.payload.id!.toString() } })
          break
        case 'Settings':
          navigate({ to: settingsRoute.to })
          break
        case 'Podcasts':
          navigate({ to: podcastsRoute.to })
          break
        case 'Downloads':
          navigate({ to: downloadsRoute.to })
          break
      }
    })
  }, [])
  return (
    <div style={{ display: 'flex' }}>
      <Sidebar/>
      <RootDiv>
        <PlayerControlsTopBar/>
        <Outlet/>
      </RootDiv>
    </div>
  )
}
