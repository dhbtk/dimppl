import React from 'react'
import { Header } from 'antd/es/layout/layout'
import { Button } from 'antd'
import { ReloadOutlined } from '@ant-design/icons'
import { useQuery } from '@tanstack/react-query'
import { podcastApi } from '../../backend/podcastApi.ts'
import { ImportPodcastButton } from './ImportPodcastButton.tsx'

export const Sidebar: React.FC = () => {
  const query = useQuery({ queryKey: ['allPodcasts'], queryFn: podcastApi.listAll })
  return (
    <>
      <Header style={{
        background: '#B1DBEF',
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'flex-start',
        padding: '0 8px'
      }}>
        <ImportPodcastButton/>
        <Button type="default" shape="circle" style={{ marginLeft: 'auto' }} icon={<ReloadOutlined/>}/>
      </Header>
      <div>
        {query.data?.join(',')}
      </div>
    </>
  )
}
