import React, { useEffect, useState } from 'react'
import { configApi } from '../../backend/configApi.ts'
import { useNavigate } from '@tanstack/react-router'
import { RootDiv } from '../../components/RootDiv.tsx'
import { AccessKeyGroup, Title, WrapperDiv } from './components.ts'
import { PrettyButton } from '../../components/PrettyButton.tsx'

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
    <RootDiv style={{ height: '100vh' }}>
      <WrapperDiv>
        <Title>Boas-vindas!</Title>
        <AccessKeyGroup>
          <label htmlFor="device_name">
            Defina um nome para este dispositivo:
          </label>
          <input
            value={deviceName}
            onChange={(e) => setDeviceName(e.target.value)}
            style={{ maxWidth: '80%' }}
          />
        </AccessKeyGroup>
        <div style={{
          display: 'flex',
          paddingTop: '15px'
        }}>
          <PrettyButton
            type="button"
            style={{ marginLeft: 'auto' }}
            disabled={loading}
            onClick={submit}
          >
            Avançar
          </PrettyButton>
        </div>
      </WrapperDiv>
    </RootDiv>
  )
}
