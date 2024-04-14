import { createRootRoute, createRoute, createRouter } from '@tanstack/react-router'
import { RootRouteComponent } from './routes/RootRouteComponent.tsx'
import { OnboardingUserAccountRoute } from './routes/onboarding/OnboardingUserAccountRoute.tsx'
import { OnboardingDeviceNameRoute } from './routes/onboarding/OnboardingDeviceNameRoute.tsx'
import { AppRoute } from './routes/app/AppRoute.tsx'
import { HomeRoute } from './routes/app/home/HomeRoute.tsx'
import { PodcastRoute } from './routes/app/podcast/PodcastRoute.tsx'
import { EpisodeWithPodcast, podcastApi } from './backend/podcastApi.ts'
import { EpisodeRoute } from './routes/app/episode/EpisodeRoute.tsx'
import { SettingsRoute } from './routes/app/manage/SettingsRoute.tsx'
import { Config, configApi } from './backend/configApi.ts'
import { PodcastsRoute } from './routes/app/manage/PodcastsRoute.tsx'
import { DownloadsRoute } from './routes/app/manage/DownloadsRoute.tsx'

export const rootRoute = createRootRoute({
  component: RootRouteComponent
})

export const onboardingUserAccountRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/onboarding/user_account',
  component: OnboardingUserAccountRoute
})
export const onboardingDeviceNameRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/onboarding/device_name',
  component: OnboardingDeviceNameRoute
})
export const appRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/app',
  component: AppRoute
})

export const appHomeRoute = createRoute({
  getParentRoute: () => appRoute,
  path: '/',
  component: HomeRoute,
  loader: async (): Promise<{
    lastPlayed: EpisodeWithPodcast | null,
    history: EpisodeWithPodcast[],
    latest: EpisodeWithPodcast[]
  }> => {
    const lastPlayed = await podcastApi.findLastPlayed()
    const history = await podcastApi.listListenHistory()
    const latest = await podcastApi.listLatestEpisodes()
    return { lastPlayed, history, latest }
  }
})

export const podcastRoute = createRoute({
  getParentRoute: () => appRoute,
  path: 'podcast/$podcastId',
  component: PodcastRoute,
  loader: async (params) => {
    const id = parseInt(params.params.podcastId, 10)
    const podcasts = await podcastApi.listAll()
    const podcast = podcasts.find(it => it.id === id)
    if (podcast !== undefined) {
      return podcast
    }
    throw new Error('podcast not found')
  }
})

export const episodeRoute = createRoute({
  getParentRoute: () => appRoute,
  path: 'episode/$episodeId',
  component: EpisodeRoute,
  loader: async (params): Promise<EpisodeWithPodcast> => {
    const id = parseInt(params.params.episodeId, 10)
    return podcastApi.getEpisodeFull(id)
  }
})

export const settingsRoute = createRoute({
  getParentRoute: () => appRoute,
  path: 'settings',
  component: SettingsRoute,
  loader: async (): Promise<Config> => configApi.load()
})

export const podcastsRoute = createRoute({
  getParentRoute: () => appRoute,
  path: 'podcasts',
  component: PodcastsRoute
})

export const downloadsRoute = createRoute({
  getParentRoute: () => appRoute,
  path: 'downloads',
  component: DownloadsRoute
})

const routeTree = rootRoute.addChildren([
  onboardingUserAccountRoute,
  onboardingDeviceNameRoute,
  appRoute.addChildren([settingsRoute, podcastsRoute, downloadsRoute, appHomeRoute, podcastRoute, episodeRoute])
])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
