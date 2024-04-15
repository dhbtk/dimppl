import styled from 'styled-components'
import React from 'react'
import { BackButton } from '../podcast/BackButton.tsx'
import { Link } from '@tanstack/react-router'
import { downloadsRoute, podcastsRoute, settingsRoute } from '../../../routeDefinitions.ts'

export const NoScrollContainer = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
`

const ToolbarContainer = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8px;
  height: 36px;
  border-bottom: 1px solid var(--gray07);
  flex-shrink: 0;
`

const ToolbarLink = styled(Link)`
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 28px;
  min-width: 28px;
  padding: 0 8px;
  cursor: default;
  background-color: var(--gray05);
  font-size: 90%;

  &[disabled] {
    color: #b4b4b4;
  }

  &:hover {
    text-decoration: none;
    background-color: var(--gray12);
  }

  &.active {
    background-color: var(--murrey);
  }
`

export const SettingsToolbar: React.FC = () => {
  return (
    <ToolbarContainer>
      <BackButton/>
      <ToolbarLink to={settingsRoute.to} replace={true}>
        Ajustes
      </ToolbarLink>
      <ToolbarLink to={podcastsRoute.to} replace={true}>
        Podcasts
      </ToolbarLink>
      <ToolbarLink to={downloadsRoute.to} replace={true}>
        Downloads
      </ToolbarLink>
    </ToolbarContainer>
  )
}
