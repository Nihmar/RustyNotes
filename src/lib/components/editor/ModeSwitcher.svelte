<script lang="ts">
    /// Mode switcher toolbar — buttons to toggle between Edit, Preview, and Reading modes.
    import { getEditorMode, setEditorMode } from '$lib/stores/ui.svelte';
    import type { EditorMode } from '$lib/types';

    const modes: { value: EditorMode; label: string }[] = [
        { value: 'edit', label: 'Edit' },
        { value: 'reading', label: 'Reading' }
    ];
</script>

<div class="mode-switcher">
    {#each modes as mode}
        <button
            class="mode-btn"
            class:active={getEditorMode() === mode.value}
            onclick={() => setEditorMode(mode.value)}
        >
            {mode.label}
        </button>
    {/each}
</div>

<style>
    .mode-switcher {
        display: flex;
        gap: 2px;
        padding: 4px;
        border-radius: 6px;
        background: var(--bg-secondary, #2a2a2a);
    }
    .mode-btn {
        padding: 4px 12px;
        font-size: 12px;
        border: none;
        background: transparent;
        color: var(--text-muted, #888);
        cursor: pointer;
        border-radius: 4px;
    }
    .mode-btn:hover {
        color: var(--text-primary, #d4d4d4);
    }
    .mode-btn.active {
        background: var(--accent, #61afef);
        color: #000;
    }
</style>
