import React, { ButtonHTMLAttributes } from 'react'
import styled from 'styled-components'

const StyledButton = styled.button`
  background-color: transparent;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 36px;
  width: 36px;
  cursor: default;
  
  &[disabled] {
    color: #b4b4b4;
  }

  &:hover {
    background-color: rgba(0, 0, 0, .05);
  }
  
  & span.material-icons-outlined {
    font-size: 24px;
  }
`

export const IconButton: React.FC<{ icon: string } & Partial<ButtonHTMLAttributes<HTMLButtonElement>>> = ({ icon, ...props }) => {
  return (
    <StyledButton {...props}>
      <span className="material-icons-outlined">{icon}</span>
    </StyledButton>
  )
}
