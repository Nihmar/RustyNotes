<script lang="ts">
    import { onMount } from 'svelte';
    import { getTags } from '$lib/commands';
    import type { TagInfo } from '$lib/types';

    let tags = $state<TagInfo[]>([]);
    let loading = $state(false);

    onMount(async () => {
        try {
            loading = true;
            tags = await getTags();
        } catch (e) {
            console.error('Failed to load tags:', e);
        } finally {
            loading = false;
        }
    });

    function handleClick(tag: TagInfo) {
        // TODO: filter file tree by tag
    }
</script>

<div class="tag-browser">
    <h3>Tags</h3>
    {#if loading}
        <p class="status">Loading...</p>
    {:else if tags.length === 0}
        <p class="status">No tags found</p>
    {:else}
        <div class="tag-list">
            {#each tags as tag}
                <button class="tag-item" onclick={() => handleClick(tag)}>
                    <span class="tag-name">#{tag.name}</span>
                    <span class="tag-count">{tag.count}</span>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .tag-browser {
        padding: 8px;
    }
    h3 {
        margin: 0 0 8px 0;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: var(--text-muted, #888);
    }
    .status {
        font-size: 12px;
        color: var(--text-muted, #888);
    }
    .tag-list {
        display: flex;
        flex-wrap: wrap;
        gap: 4px;
    }
    .tag-item {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 2px 8px;
        font-size: 12px;
        border: none;
        background: var(--bg-secondary, #333);
        color: var(--accent, #61afef);
        cursor: pointer;
        border-radius: 4px;
    }
    .tag-item:hover {
        background: var(--hover-bg, #444);
    }
    .tag-count {
        font-size: 10px;
        color: var(--text-muted, #888);
    }
</style>
