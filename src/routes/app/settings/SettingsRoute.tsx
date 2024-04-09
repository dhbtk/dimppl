import React from 'react'
import styled from 'styled-components'
import { BackButton } from '../podcast/BackButton.tsx'
import { onboardingUserAccountRoute, settingsRoute } from '../../../routeDefinitions.ts'
import { Link } from '@tanstack/react-router'

const RootDiv = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 100vh;
  overflow: auto;
`

const WrapperDiv = styled.div`
  padding: 4px 12px 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
`

const Header = styled.h1`
  font-size: 24px;
`

export const SettingsRoute: React.FC = () => {
  const config = settingsRoute.useLoaderData()
  return (
    <RootDiv>
      <WrapperDiv>
        <BackButton/>
        <Header>Configurações</Header>
        <pre>{JSON.stringify(config, null, 2)}</pre>
        <Link to={onboardingUserAccountRoute.to}>onboardingUserAccountRoute</Link>
      </WrapperDiv>
    </RootDiv>
  )
}
