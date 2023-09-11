import React, { useCallback, useState } from 'react'
import { ToolbarButton } from './ToolbarButton.tsx'
import styled, { css, keyframes } from 'styled-components'
import { podcastApi } from '../../backend/podcastApi.ts'

const rotate = keyframes`
  to {
    transform: rotate(360deg);
  }
`

const SpinningButton = styled(ToolbarButton)`
  ${props => props.disabled ? css`animation: ${rotate} 1s linear infinite;` : ''}
`


export const SyncPodcastsButton: React.FC = () => {
  const [loading, setLoading] = useState(false)
  const submit = useCallback(() => {
    setLoading(true)
    podcastApi.syncPodcasts().then(() => setLoading(false), () => setLoading(false))
  }, [setLoading])
  return (
    <SpinningButton type="button" disabled={loading} onClick={() => !loading && submit()}>
      <span className="material-icons-outlined">refresh</span>
    </SpinningButton>
  )
}
