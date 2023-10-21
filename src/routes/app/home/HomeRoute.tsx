import React from 'react'
import { appHomeRoute } from '../../../routeDefinitions.ts'
import { LastPlayedCard } from './LastPlayedCard.tsx'

export const HomeRoute: React.FC = () => {
  const { lastPlayed } = appHomeRoute.useLoader()
  return (
    <div style={{ flex: '1', display: 'flex', flexDirection: 'column', gap: 8, maxHeight: '100vh' }}>
      {lastPlayed !== null && (
        <>
          <LastPlayedCard lastPlayed={lastPlayed} />
          <hr style={{ margin: 8 }} />
        </>
      )}
    </div>
  )
}
