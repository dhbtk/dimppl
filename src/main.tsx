import React from 'react'
import ReactDOM from 'react-dom/client'
import { OnboardingUserAccountRoute } from './routes/onboarding/OnboardingUserAccountRoute.tsx'
import { App } from 'antd'

import { RootRoute, Route, Router, RouterProvider } from '@tanstack/react-router'
import { AppRoute } from './routes/app/AppRoute.tsx'
import { RootRouteComponent } from './routes/RootRouteComponent.tsx'
import { OnboardingDeviceNameRoute } from './routes/onboarding/OnboardingDeviceNameRoute.tsx'

const rootRoute = new RootRoute({
  component: RootRouteComponent
})

const onboardingUserAccountRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/onboarding/user_account',
  component: OnboardingUserAccountRoute
})
const onboardingDeviceNameRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/onboarding/device_name',
  component: OnboardingDeviceNameRoute
})
const appRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/app',
  component: AppRoute
})

const routeTree = rootRoute.addChildren([
  onboardingUserAccountRoute,
  onboardingDeviceNameRoute,
  appRoute
])

const router = new Router({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App>
      <RouterProvider router={router}/>
    </App>
  </React.StrictMode>,
)
