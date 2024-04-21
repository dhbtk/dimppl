import React from 'react'
import { ToolbarButton } from '../ToolbarButton.tsx'

export const BackButton: React.FC = () => {
  return (
    <ToolbarButton type="button" onClick={() => history.back()}>
      <span className="material-icons-outlined" style={{ fontSize: 12 }}>
        arrow_back_ios_new
      </span>
    </ToolbarButton>
  )
}
