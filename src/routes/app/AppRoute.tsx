import React from 'react'
import { Outlet } from '@tanstack/react-router'
import { Sidebar } from './Sidebar.tsx'
import { PlayerControlsTopBar } from './PlayerControlsTopBar.tsx'

export const AppRoute: React.FC = () => {
  return (
    <div style={{ display: 'flex' }}>
      <Sidebar/>
      <div style={{ flex: '1', display: 'flex', flexDirection: 'column', maxHeight: 'calc(100vh - 2px)' }}>
        <PlayerControlsTopBar/>
        <Outlet/>
      </div>
    </div>
  )
}
