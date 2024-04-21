import React, { ButtonHTMLAttributes } from 'react'
import styled from 'styled-components'

const StyledButton = styled.button`
  background-color: rgba(0, 0, 0, .78);
  color: rgba(255, 255, 255, .95);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 52px;
  width: 52px;
  cursor: default;
  
  &[disabled] {
    color: #b4b4b4;
  }
  
  & span.material-icons-outlined {
    font-size: 46px;
  }
`

export const ImageOverlayButton: React.FC<{ icon: string } & Partial<ButtonHTMLAttributes<HTMLButtonElement>>> = ({ icon, ...props }) => {
  return (
    <StyledButton type="button" {...props}>
      <span className="material-icons-outlined">{icon}</span>
    </StyledButton>
  )
}
