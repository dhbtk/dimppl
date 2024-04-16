import styled from 'styled-components'
import React from 'react'
import { BackButton } from '../podcast/BackButton.tsx'
import { Link } from '@tanstack/react-router'
import { downloadsRoute, podcastsRoute, settingsRoute } from '../../../routeDefinitions.ts'

export const NoScrollContainer = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
  max-height: 100%;
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
  background-color: var(--gray12);
  font-size: 90%;
  border: 2px ridge var(--gray12);

  &[disabled] {
    color: #b4b4b4;
  }

  &:hover {
    text-decoration: none;
    background-color: var(--gray12);
  }

  &.active {
    background-color: var(--murrey);
    border-style: groove;
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
export const CoolTable = styled.table`
  font-size: 90%;
  border: 1px solid var(--gray07);
  border-collapse: collapse;
  width: 100%;

  td, th {
    padding: 2px 6px;
    height: 3.1em;
    border-bottom: 1px solid var(--gray07);
  }

  th {
    text-align: start;
    white-space: nowrap;
    padding: 2px 8px;
  }

  th:not(:first-child):not(:last-child) {
    border-left: 1px solid var(--gray07);
    border-right: 1px solid var(--gray07);
  }
  
  td {
    vertical-align: middle;
  }

  td.tiny {
    white-space: nowrap;
    text-align: right;
    width: 1em;
  }

  td.selectable {
    -webkit-user-select: text;
  }

  tr:nth-child(odd) {
    background-color: var(--gray05);
  }

  tr:nth-child(even) {
    background-color: var(--gray07);
  }
`
export const TableContainer = styled.div`
  overflow-y: auto;
  flex: 1;
  padding: 8px;
`
