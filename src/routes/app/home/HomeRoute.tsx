import React from 'react'
import { appHomeRoute } from '../../../routeDefinitions.ts'
import { LastPlayedCard } from './LastPlayedCard.tsx'
import { ListenHistoryTileList } from './ListenHistoryTileList.tsx'
import { RecentEpisodesList } from './RecentEpisodesList.tsx'

export const HomeRoute: React.FC = () => {
  const { lastPlayed } = appHomeRoute.useLoaderData()
  return (
    <div style={{
      flex: '1',
      display: 'flex',
      flexDirection: 'column',
      gap: 8,
      height: '100%',
      overflowY: 'auto',
      overscrollBehaviorY: 'initial'
    }}>
      {lastPlayed !== null && (
        <>
          <LastPlayedCard lastPlayed={lastPlayed}/>
          <hr style={{ margin: 8 }}/>
        </>
      )}
      <ListenHistoryTileList/>
      <hr style={{ margin: '0 8px' }}/>
      <RecentEpisodesList/>
    </div>
  )
}
