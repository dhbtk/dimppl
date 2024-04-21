import styled from 'styled-components'

export const RootDiv = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
  max-height: 100vh;
  border-bottom-right-radius: 9px;
  @media (prefers-color-scheme: dark) {
    background: transparent;
  }
  @media (prefers-color-scheme: light) {
    background: var(--primary-lightest);
  }
`
