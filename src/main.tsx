import React from 'react'
import ReactDOM from 'react-dom/client'

import { RouterProvider } from '@tanstack/react-router'

import 'normalize.css'
import './app.css'
import 'material-icons/iconfont/material-icons.css'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { router } from './routeDefinitions.ts'
import { listen } from '@tauri-apps/api/event'
import { DownloadContextProvider } from './routes/DownloadContextProvider.tsx'
import { PlayerContextProvider } from './routes/PlayerContextProvider.tsx'
import { invoke } from '@tauri-apps/api/core'

const queryClient = new QueryClient()

listen<string>('invalidate-cache', event => {
  queryClient.invalidateQueries({ queryKey: [event.payload] })
})

invoke<void>('set_up_media_controls')

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <DownloadContextProvider>
        <PlayerContextProvider>
          <RouterProvider router={router}/>
        </PlayerContextProvider>
      </DownloadContextProvider>
    </QueryClientProvider>
  </React.StrictMode>,
)

document.addEventListener('contextmenu', ev => {
  ev.preventDefault()
})
