import React, { ButtonHTMLAttributes } from 'react'
import { ToolbarButton } from '../ToolbarButton.tsx'

export const ToolbarIconButton: React.FC<{ icon: string } & Partial<ButtonHTMLAttributes<HTMLButtonElement>>> = ({
  icon,
  ...props
}) => {
  return (
    <ToolbarButton {...props}>
      <span className="material-icons-outlined">{icon}</span>
    </ToolbarButton>
  )
}
