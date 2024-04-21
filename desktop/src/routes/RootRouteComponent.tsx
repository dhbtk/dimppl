import React, { useEffect } from 'react'
import { configApi } from '../backend/configApi.ts'
import { Outlet, ScrollRestoration, useNavigate } from '@tanstack/react-router'
import { appHomeRoute, onboardingUserAccountRoute } from '../routeDefinitions.ts'

export const RootRouteComponent: React.FC = () => {
  const navigate = useNavigate({ from: '/' })
  useEffect(() => {
    configApi.load().then(configData => {
      console.log(configData)
      if (configData.accessToken.length !== 0 && configData.userAccessKey.length !== 0) {
        return navigate({ to: appHomeRoute.to })
      } else {
        return navigate({ to: onboardingUserAccountRoute.to })
      }
    })
  }, [])
  return (
    <>
      <ScrollRestoration />
      <Outlet/>
    </>
  )
}
