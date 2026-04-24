<script lang="ts">
    import { onMount } from 'svelte';
    import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
    import EditorPane from '$lib/components/editor/EditorPane.svelte';
    import WelcomeScreen from '$lib/components/WelcomeScreen.svelte';
    import { activeNotebook } from '$lib/stores/notebook.svelte';
    import { sidebarVisible } from '$lib/stores/ui.svelte';
    import { setupFileWatcher } from '$lib/events';

    onMount(() => {
        setupFileWatcher();
    });
</script>

<div class="app" class:sidebar-hidden={!sidebarVisible}>
    {#if activeNotebook}
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
        background: var(--bg-primary, #1e1e1e);
        color: var(--text-primary, #d4d4d4);
    }

    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }
</style>
