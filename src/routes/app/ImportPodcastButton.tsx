import React from 'react'
import { PlusOutlined } from '@ant-design/icons'
import { Button } from 'antd'

export const ImportPodcastButton: React.FC = () => {
  return (
    <>
      <Button type="default" shape="circle" icon={<PlusOutlined/>}/>
    </>
  )
}
