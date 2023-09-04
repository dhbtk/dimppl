import React from 'react'
import ReactDOM from 'react-dom/client'
import { OnboardingUserAccountRoute } from './routes/onboarding/OnboardingUserAccountRoute.tsx'

import { RootRoute, Route, Router, RouterProvider } from '@tanstack/react-router'
import { AppRoute } from './routes/app/AppRoute.tsx'
import { RootRouteComponent } from './routes/RootRouteComponent.tsx'
import { OnboardingDeviceNameRoute } from './routes/onboarding/OnboardingDeviceNameRoute.tsx'

import 'normalize.css'
import './app.css'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

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

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router}/>
    </QueryClientProvider>
  </React.StrictMode>,
)
