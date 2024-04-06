import React from 'react'
import { Outlet } from '@tanstack/react-router'
import { Sidebar } from './Sidebar.tsx'
import { PlayerControlsTopBar } from './PlayerControlsTopBar.tsx'
import styled from 'styled-components'

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
