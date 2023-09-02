import React, { useEffect, useState } from 'react'
import { Button, Input, Radio, RadioChangeEvent, Space, Typography } from 'antd'
import { configApi } from '../../backend/configApi.ts'
import { useNavigate } from '@tanstack/react-router'

export const OnboardingUserAccountRoute: React.FC = () => {
  const [selectedOption, setSelectedOption] = useState('')
  const [existingKey, setExistingKey] = useState('')
  const [loading, setLoading] = useState(false)
  const navigate = useNavigate({ from: '/onboarding/user_account' })

  useEffect(() => {
    configApi.load().then((configData) => {
      if (configData.userAccessKey.length > 0) {
        setSelectedOption('existing')
        setExistingKey(configData.userAccessKey)
      }
    })
  }, [])

  const onSelectionChange = (e: RadioChangeEvent) => {
    setSelectedOption(e.target.value)
  }

  const submit = async () => {
    setLoading(true)
    if (selectedOption === 'new') {
      await configApi.registerNewUser()
    } else {
      let config = await configApi.load()
      config.userAccessKey = existingKey
      await configApi.save(config)
    }
    navigate({ to: '/onboarding/device_name' })
    setLoading(false)
  }

  return (
    <Space direction="vertical" size="small" style={{ padding: '0 15px' }}>
      <Typography.Title>Boas-vindas!</Typography.Title>
      <Typography.Text>
        Para sincronizar o progresso entre dispositivos, o Dimppl criará um código de acesso para você. Caso você já
        use o Dimppl em outro dispositivo, você pode digitar seu código de acesso e entrar na sua conta.
      </Typography.Text>
      <Radio.Group onChange={onSelectionChange} value={selectedOption}>
        <Space direction="vertical">
          <Radio value="new">Não tenho uma conta ou gostaria de criar uma nova conta</Radio>
          <Radio value="existing">Tenho uma conta e gostaria de usá-la neste dispositivo</Radio>
        </Space>
      </Radio.Group>
      <Input
        value={existingKey}
        placeholder="XXXXXXXX-XXXX-XXXX-XXXXXXXXXX-XX"
        onChange={(e) => setExistingKey(e.target.value)}
        style={{ maxWidth: '80%', visibility: selectedOption === 'existing' ? 'visible' : 'hidden' }}
      />
      <div style={{
        display: 'flex',
        paddingTop: '15px'
      }}>
        <Button
          type="primary"
          style={{ marginLeft: 'auto' }}
          disabled={loading || selectedOption.length === 0}
          onClick={submit}
        >
          Avançar
        </Button>
      </div>
    </Space>
  )
}
