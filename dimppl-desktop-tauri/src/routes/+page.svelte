<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<p>{config}</p>
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  let config = 'empty'

  onMount(() => {
    invoke('get_config').then(result => {
      config = JSON.stringify(result)
    })
  })
</script>
