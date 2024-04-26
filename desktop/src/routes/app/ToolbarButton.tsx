import styled from 'styled-components'

export const ToolbarButton = styled.button`
  background-color: transparent;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 28px;
  width: 28px;
  cursor: default;

  &[disabled], &:active {
    color: #b4b4b4;
  }

  &:hover {
    background-color: var(--gray05);
  }
`
