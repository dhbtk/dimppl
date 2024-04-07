import React, { useEffect } from 'react'
import { Outlet, useNavigate } from '@tanstack/react-router'
import { Sidebar } from './Sidebar.tsx'
import { PlayerControlsTopBar } from './PlayerControlsTopBar.tsx'
import styled from 'styled-components'
import { appRoute, router, settingsRoute } from '../../routeDefinitions.ts'

const RootDiv = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
  max-height: 100vh;
  border-top-right-radius: 9px;
  border-bottom-right-radius: 9px;
  @media (prefers-color-scheme: dark) {
    background: transparent;
  }
  @media (prefers-color-scheme: light) {
    background: #FFF;
  }
`

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
