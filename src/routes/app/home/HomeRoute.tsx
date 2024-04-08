import React from 'react'
import { appHomeRoute } from '../../../routeDefinitions.ts'
import { LastPlayedCard } from './LastPlayedCard.tsx'
import { ListenHistoryTileList } from './ListenHistoryTileList.tsx'

export const HomeRoute: React.FC = () => {
  const { lastPlayed } = appHomeRoute.useLoaderData()
  return (
    <div style={{ flex: '1', display: 'flex', flexDirection: 'column', gap: 8, maxHeight: '100vh', overflowY: 'auto' }}>
      {lastPlayed !== null && (
        <>
          <LastPlayedCard lastPlayed={lastPlayed}/>
          <hr style={{ margin: 8 }}/>
        </>
      )}
      <ListenHistoryTileList/>
    </div>
  )
}
