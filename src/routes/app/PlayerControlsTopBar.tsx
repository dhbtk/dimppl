import React from 'react'
import styled from 'styled-components'

const TopBar = styled.div`
  height: 48px;
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 0 8px;
  border-bottom: 1px solid #ECECEC;
`

export const PlayerControlsTopBar: React.FC = () => {
  return (
    <TopBar data-tauri-drag-region={true}>
      player controls eventually
    </TopBar>
  )
}
