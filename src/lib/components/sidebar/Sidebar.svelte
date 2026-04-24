<script lang="ts">
    import NotebookSelector from './NotebookSelector.svelte';
    import FileTree from './FileTree.svelte';
    import SearchPanel from './SearchPanel.svelte';
    import TagBrowser from './TagBrowser.svelte';
    import { getSidebarVisible, toggleSidebar } from '$lib/stores/ui.svelte';
    import { getActiveNotebook } from '$lib/stores/notebook.svelte';

    let activePanel = $state<'files' | 'search' | 'tags'>('files');
</script>

{#if getActiveNotebook()}
    <button class="hamburger" onclick={toggleSidebar} aria-label="Toggle sidebar">
        ☰
    </button>
{/if}

<div class="sidebar" class:collapsed={!getSidebarVisible()}>
    <NotebookSelector />
    <div class="panel-tabs">
        <button class="panel-tab" class:active={activePanel === 'files'} onclick={() => (activePanel = 'files')}>Files</button>
        <button class="panel-tab" class:active={activePanel === 'search'} onclick={() => (activePanel = 'search')}>Search</button>
        <button class="panel-tab" class:active={activePanel === 'tags'} onclick={() => (activePanel = 'tags')}>Tags</button>
    </div>
    <div class="panel-content">
        {#if activePanel === 'files'}
            <FileTree />
        {:else if activePanel === 'search'}
            <SearchPanel />
        {:else}
            <TagBrowser />
        {/if}
    </div>
</div>

<style>
    .hamburger {
        display: none;
        position: absolute;
        top: 4px;
        left: 4px;
        z-index: 100;
        background: var(--bg-secondary, #333);
        border: 1px solid var(--border-color, #555);
        color: inherit;
        padding: 4px 8px;
        font-size: 18px;
        cursor: pointer;
        border-radius: 4px;
    }
    .sidebar {
        display: flex;
        flex-direction: column;
        height: 100%;
        background: var(--bg-secondary, #252525);
        border-right: 1px solid var(--border-color, #333);
        min-width: 220px;
        max-width: 400px;
    }
    .sidebar.collapsed {
        display: none;
    }
    .panel-tabs {
        display: flex;
        border-bottom: 1px solid var(--border-color, #333);
    }
    .panel-tab {
        flex: 1;
        padding: 6px 4px;
        font-size: 11px;
        border: none;
        background: none;
        color: var(--text-muted, #888);
        cursor: pointer;
        text-transform: uppercase;
        letter-spacing: 0.3px;
    }
    .panel-tab.active {
        color: var(--accent, #61afef);
        border-bottom: 2px solid var(--accent, #61afef);
    }
    .panel-tab:hover {
        color: var(--text-primary, #d4d4d4);
    }
    .panel-content {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    @media (max-width: 768px) {
        .hamburger {
            display: block;
        }
        .sidebar {
            position: fixed;
            top: 0;
            left: 0;
            bottom: 0;
            z-index: 50;
            width: 280px;
            max-width: 80vw;
        }
        .sidebar.collapsed {
            display: none;
        }
    }
</style>
