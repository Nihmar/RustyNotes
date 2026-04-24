<script lang="ts">
    import { onMount } from 'svelte';
    import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
    import EditorPane from '$lib/components/editor/EditorPane.svelte';
    import WelcomeScreen from '$lib/components/WelcomeScreen.svelte';
    import { getActiveNotebook } from '$lib/stores/notebook.svelte';
    import { getSidebarVisible, getTheme } from '$lib/stores/ui.svelte';
    import { setupFileWatcher } from '$lib/events';

    onMount(() => {
        setupFileWatcher();
    });
</script>

<div class="app" class:sidebar-hidden={!getSidebarVisible()} class:theme-light={getTheme() === 'light'}>
    {#if getActiveNotebook()}
        <Sidebar />
        <EditorPane />
    {:else}
        <WelcomeScreen />
    {/if}
</div>

<style>
    .app {
        display: flex;
        width: 100vw;
        height: 100vh;
        overflow: hidden;
        --bg-primary: #1e1e1e;
        --bg-secondary: #2a2a2a;
        --text-primary: #d4d4d4;
        --text-muted: #888;
        --accent: #61afef;
        --border-color: #444;
        --shadow: rgba(0, 0, 0, 0.3);
        background: var(--bg-primary);
        color: var(--text-primary);
    }

    .app.theme-light {
        --bg-primary: #ffffff;
        --bg-secondary: #f5f5f5;
        --text-primary: #1e1e1e;
        --text-muted: #666;
        --accent: #0366d6;
        --border-color: #d1d5db;
        --shadow: rgba(0, 0, 0, 0.08);
    }

    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    :global(body) {
        background: var(--bg-primary);
    }
</style>
