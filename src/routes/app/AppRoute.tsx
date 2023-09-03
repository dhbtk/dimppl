import React from 'react'
import { Layout } from 'antd'
import Sider from 'antd/es/layout/Sider'
import { Content, Header } from 'antd/es/layout/layout'
import { Outlet } from '@tanstack/react-router'
import { Sidebar } from './Sidebar.tsx'

export const AppRoute: React.FC = () => {
  return (
    <Layout>
      <Sider style={{
        overflow: 'auto',
        height: '100vh',
        position: 'fixed',
        left: 0,
        top: 0,
        bottom: 0,
        background: '#84C5E6'
      }}>
        <Sidebar/>
      </Sider>
      <Layout>
        <Header>
          Header!!
        </Header>
        <Content>
          <Outlet/>
        </Content>
      </Layout>
    </Layout>
  )
}
