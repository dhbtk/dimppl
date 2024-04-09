import React, { useEffect } from 'react'
import { Outlet, useNavigate } from '@tanstack/react-router'
import { Sidebar } from './Sidebar.tsx'
import { PlayerControlsTopBar } from './PlayerControlsTopBar.tsx'
import { settingsRoute } from '../../routeDefinitions.ts'
import { RootDiv } from '../../components/RootDiv.tsx'

export const AppRoute: React.FC = () => {
  const navigate = useNavigate()
  useEffect(() => {
    document.addEventListener('keydown', e => {
      if ((e.ctrlKey || e.metaKey) && e.key === ',') {
        navigate({ to: settingsRoute.to })
      }
    })
  })
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
