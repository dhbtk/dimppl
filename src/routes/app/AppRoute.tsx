import React from 'react'
import { Outlet } from '@tanstack/react-router'
import { Sidebar } from './Sidebar.tsx'

export const AppRoute: React.FC = () => {
  return (
    <div style={{ display: 'flex' }}>
      <Sidebar/>
      <div style={{ flex: '1' }}>
        <Outlet/>
      </div>
    </div>
  )
}
