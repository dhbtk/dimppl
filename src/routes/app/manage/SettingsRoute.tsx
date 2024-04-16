import React from 'react'
import styled from 'styled-components'
import { BackButton } from '../podcast/BackButton.tsx'
import { onboardingUserAccountRoute, settingsRoute } from '../../../routeDefinitions.ts'
import { Link } from '@tanstack/react-router'
import { CoolTable, NoScrollContainer, SettingsToolbar, TableContainer } from './shared.tsx'
import { Config } from '../../../backend/configApi.ts'

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
    <NoScrollContainer>
      <SettingsToolbar/>
      <TableContainer>
        <CoolTable>
          <thead>
          <tr>
            <th>
              Chave
            </th>
            <th>
              Valor
            </th>
          </tr>
          </thead>
          <tbody>
          {Object.keys(config).map(key => (
            <tr key={key}>
              <td className="tiny">
                {key}
              </td>
              <td className="selectable">
                {config[key as keyof Config]}
              </td>
            </tr>
          ))}
          </tbody>
        </CoolTable>
      </TableContainer>
      <Link to={onboardingUserAccountRoute.to}>onboardingUserAccountRoute</Link>
    </NoScrollContainer>
  )
}
