import React, { useEffect, useState } from 'react'
import { configApi } from '../../backend/configApi.ts'
import { Button, Input, Typography } from 'antd'
import { useNavigate } from '@tanstack/react-router'

export const OnboardingDeviceNameRoute: React.FC = () => {
  const [deviceName, setDeviceName] = useState('')
  const [loading, setLoading] = useState(false)
  const navigate = useNavigate({ from: '/onboarding/device_name' })
  useEffect(() => {
    configApi.load().then(config => setDeviceName(config.deviceName))
  }, [])

  const submit = async () => {
    setLoading(true)
    try {
      await configApi.registerDevice(deviceName)
      navigate({ to: '/' })
      setLoading(false)
    } catch (e) {
      console.log(e)
      alert(e)
      setLoading(false)
    }
  }
  return (
    <div style={{ padding: '0 15px', display: 'flex', flexDirection: 'column', gap: '10px' }}>
      <Typography.Title>Boas-vindas!</Typography.Title>
      <Typography.Text>
        Defina um nome para este dispositivo:
      </Typography.Text>
      <Input
        value={deviceName}
        onChange={(e) => setDeviceName(e.target.value)}
        style={{ maxWidth: '80%' }}
      />
      <div style={{
        display: 'flex',
        paddingTop: '15px'
      }}>
        <Button
          type="primary"
          style={{ marginLeft: 'auto' }}
          disabled={loading}
          onClick={submit}
        >
          Avançar
        </Button>
      </div>
    </div>
  )
}
