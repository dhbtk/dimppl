import React, { useCallback, useEffect, useState } from 'react'
import { ToolbarButton } from '../ToolbarButton.tsx'
import styled, { css, keyframes } from 'styled-components'
import { podcastApi } from '../../../backend/podcastApi.ts'
import { listen } from '@tauri-apps/api/event'

const rotate = keyframes`
  to {
    transform: rotate(360deg);
  }
`

const SpinningButton = styled(ToolbarButton)`
  & .material-icons-outlined {
    ${props => props.disabled ? css`animation: ${rotate} 1s linear infinite;` : ''}
  }
`

export const SyncPodcastsButton: React.FC = () => {
  const [loading, setLoading] = useState(false)
  const submit = useCallback(() => {
    setLoading(true)
    podcastApi.syncPodcasts()
  }, [setLoading])
  useEffect(() => {
    listen('sync-podcasts-done', () => {
      setLoading(false)
    })
    listen('sync-podcasts-start', () => {
      setLoading(true)
    })
  }, [setLoading])
  return (
    <SpinningButton type="button" disabled={loading} onClick={() => !loading && submit()}>
      <span className="material-icons-outlined">refresh</span>
    </SpinningButton>
  )
}
