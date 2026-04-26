<script lang="ts">
    import { getTabs, getActiveTabIndex, openTab, closeTab, setActive } from '$lib/stores/tabs.svelte';

    function handleClose(index: number, e: MouseEvent) {
        e.stopPropagation();
        closeTab(index);
    }

    function handleCloseAll() {
        while (getTabs().length > 0) {
            closeTab(getTabs().length - 1);
        }
    }
</script>

{#if getTabs().length > 0}
    <div class="tab-bar">
        {#each getTabs() as tab, i}
            <div class="tab-row">
                <button
                    class="tab"
                    class:active={i === getActiveTabIndex()}
                    onclick={() => setActive(i)}
                >
                    <span class="tab-title">{tab.title}</span>
                    {#if tab.isDirty}
                        <span class="tab-dirty">●</span>
                    {/if}
                </button>
                <button class="tab-close" onclick={(e) => handleClose(i, e)}>×</button>
            </div>
        {/each}
        {#if getTabs().length > 1}
            <button class="tab close-all" onclick={handleCloseAll} title="Close all tabs">
                ✕
            </button>
        {/if}
    </div>
{/if}

<style>
    .tab-bar {
        display: flex;
        background: var(--bg-secondary, #252525);
        border-bottom: 1px solid var(--border-color, #333);
        overflow-x: auto;
        scrollbar-width: none;
    }
    .tab-bar::-webkit-scrollbar { display: none; }
    .tab-row {
        display: flex;
        align-items: center;
    }
    .tab {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 4px 12px;
        font-size: 12px;
        border: none;
        border-right: 1px solid var(--border-color, #333);
        background: none;
        color: var(--text-muted, #888);
        cursor: pointer;
        white-space: nowrap;
        min-width: 0;
    }
    .tab.active {
        background: var(--bg-primary, #1e1e1e);
        color: var(--text-primary, #d4d4d4);
    }
    .tab:hover {
        background: var(--bg-primary, #1e1e1e);
    }
    .tab-title {
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 120px;
    }
    .tab-dirty {
        color: var(--accent, #61afef);
        font-size: 8px;
    }
    .tab-close {
        cursor: pointer;
        opacity: 0.5;
        font-size: 14px;
        padding: 0 2px;
    }
    .tab-close:hover {
        opacity: 1;
        color: #e06c75;
    }
</style>
