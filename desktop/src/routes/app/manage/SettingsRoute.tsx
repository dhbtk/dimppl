import React from 'react'
import { onboardingUserAccountRoute, settingsRoute } from '../../../routeDefinitions.ts'
import { Link } from '@tanstack/react-router'
import { CoolTable, NoScrollContainer, SettingsToolbar, TableContainer } from './shared.tsx'
import { Config } from '../../../backend/configApi.ts'

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
