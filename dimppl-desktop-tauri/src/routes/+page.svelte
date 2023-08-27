<p>{content}</p>
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import type { Config } from '$lib/models'
  let content = ''

  onMount(() => {
    invoke<Config>('get_config').then(result => {
      if (result.accessToken.length === 0 || result.userAccessKey.length === 0) {
        content = 'go to onboarding you fool!'
      } else {
        content = 'go elsewhere'
      }
    })
  })
</script>
