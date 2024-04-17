import React, { ChangeEvent, useEffect, useState } from 'react'
import { configApi } from '../../backend/configApi.ts'
import { useNavigate } from '@tanstack/react-router'
import { RootDiv } from '../../components/RootDiv.tsx'
import { PrettyButton } from '../../components/PrettyButton.tsx'
import { AccessKeyGroup, RadioGroup, Title, WrapperDiv } from './components.ts'

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

  const onSelectionChange = (e: ChangeEvent<HTMLInputElement>) => {
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
    <RootDiv style={{ height: '100vh' }}>
      <WrapperDiv>
        <Title>Boas-vindas!</Title>
        <p>
          Para sincronizar o progresso entre dispositivos, o Dimppl criará um código de acesso para você. Caso você já
          use o Dimppl em outro dispositivo, você pode digitar seu código de acesso e entrar na sua conta.
        </p>
        <RadioGroup>
          <label htmlFor="opt_new">
            <input type="radio" name="account_option" id="opt_new" value="new" checked={selectedOption === 'new'}
                   onChange={onSelectionChange}/>
            <span>Não tenho uma conta ou gostaria de criar uma nova conta</span>
          </label>
          <label htmlFor="opt_existing">
            <input type="radio" name="account_option" id="opt_new" value="existing"
                   checked={selectedOption === 'existing'}
                   onChange={onSelectionChange}/>
            <span>Tenho uma conta e gostaria de usá-la neste dispositivo</span>
          </label>
        </RadioGroup>
        <AccessKeyGroup style={{ visibility: selectedOption === 'existing' ? 'visible' : 'hidden' }}>
          <label htmlFor="existing_key">
            Código de acesso da conta:
          </label>
          <input
            id="existing_key"
            value={existingKey}
            placeholder="XXXXXXXX-XXXX-XXXX-XXXXXXXXXX-XX"
            onChange={(e) => setExistingKey(e.target.value)}
            style={{ width: '100%' }}
          />
        </AccessKeyGroup>
        <div style={{
          display: 'flex',
          paddingTop: '15px'
        }}>
          <PrettyButton
            type="button"
            style={{ marginLeft: 'auto' }}
            disabled={loading || selectedOption.length === 0}
            onClick={submit}
          >
            Avançar
          </PrettyButton>
        </div>
      </WrapperDiv>
    </RootDiv>
  )
}
