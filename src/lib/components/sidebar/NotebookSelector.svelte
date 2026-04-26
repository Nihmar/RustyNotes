<script lang="ts">
    /// Notebook selector sidebar panel.
    /// Shows the active notebook, a list of recent notebooks for quick switching,
    /// and actions to open a folder or create a new notebook.
    import { onMount } from 'svelte';
    import { getActiveNotebook, getRecentNotebooks, isLoading, setActive, setRecent, setLoading } from '$lib/stores/notebook.svelte';
    import { listNotebooks, openNotebook, createNotebook } from '$lib/commands';
    import { open as openDialog } from '@tauri-apps/plugin-dialog';
    import type { NotebookInfo } from '$lib/types';

    let showNewForm = $state(false);
    let newName = $state('');
    let newPath = $state('');
    let error = $state('');

    onMount(async () => {
        try {
            setLoading(true);
            const books = await listNotebooks();
            setRecent(books);
        } catch (e) {
            console.error('Failed to load notebooks:', e);
        } finally {
            setLoading(false);
        }
    });

    async function handleOpenRecent(nb: NotebookInfo) {
        try {
            setLoading(true);
            error = '';
            const notebook = await openNotebook(nb.path);
            setActive(notebook);
        } catch (e: any) {
            error = e;
        } finally {
            setLoading(false);
        }
    }

    async function handleOpenFolder() {
        try {
            const selected = await openDialog({ directory: true, multiple: false });
            if (!selected) return;
            setLoading(true);
            error = '';
            const notebook = await openNotebook(selected);
            setActive(notebook);
        } catch (e: any) {
            error = e;
        } finally {
            setLoading(false);
        }
    }

    async function handleCreate() {
        if (!newName.trim() || !newPath.trim()) return;
        try {
            setLoading(true);
            error = '';
            const notebook = await createNotebook(newName.trim(), newPath.trim());
            setActive(notebook);
            showNewForm = false;
            newName = '';
            newPath = '';
        } catch (e: any) {
            error = e;
        } finally {
            setLoading(false);
        }
    }

    async function handlePickNewFolder() {
        try {
            const selected = await openDialog({ directory: true, multiple: false });
            if (selected) {
                newPath = selected;
            }
        } catch (e: any) {
            error = e;
        }
    }
</script>

<div class="notebook-selector">
    <h3>Notebook</h3>

    {#if getActiveNotebook()}
        {@const nb = getActiveNotebook()!}
        <div class="active-notebook">
            <span class="nb-icon">📓</span>
            <span class="nb-name">{nb.name}</span>
        </div>
    {/if}

    {#if error}
        <p class="error">{error}</p>
    {/if}

    {#if getRecentNotebooks().length > 0}
        <div class="recent-list">
            <p class="label">Recent</p>
            {#each getRecentNotebooks() as nb}
                <button class="recent-item" onclick={() => handleOpenRecent(nb)} disabled={isLoading()}>
                    {nb.name}
                </button>
            {/each}
        </div>
    {/if}

    <div class="actions">
        <button onclick={handleOpenFolder}>
            Open Folder
        </button>
        <button onclick={() => (showNewForm = !showNewForm)}>
            New Notebook
        </button>
    </div>

    {#if showNewForm}
        <form class="form" onsubmit={(e) => { e.preventDefault(); handleCreate(); }}>
            <input type="text" placeholder="Notebook name..." bind:value={newName} />
            <div class="path-row">
                <input type="text" placeholder="Notebook path..." bind:value={newPath} />
                <button type="button" class="browse-btn" onclick={handlePickNewFolder}>...</button>
            </div>
            <button type="submit" disabled={isLoading()}>Create</button>
        </form>
    {/if}
</div>

<style>
    .notebook-selector {
        padding: 8px;
        border-bottom: 1px solid var(--border-color, #333);
    }
    h3 {
        margin: 0 0 8px 0;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: var(--text-muted, #888);
    }
    .active-notebook {
        display: flex;
        align-items: center;
        gap: 6px;
        padding: 4px 6px;
        font-size: 13px;
        font-weight: 600;
    }
    .nb-icon {
        font-size: 14px;
    }
    .error {
        color: #e06c75;
        font-size: 12px;
        margin: 4px 0;
    }
    .label {
        font-size: 11px;
        color: var(--text-muted, #888);
        margin: 4px 0;
    }
    .recent-list {
        margin-top: 8px;
    }
    .recent-item {
        display: block;
        width: 100%;
        background: none;
        border: none;
        color: inherit;
        cursor: pointer;
        padding: 4px 6px;
        font-size: 13px;
        text-align: left;
        border-radius: 4px;
    }
    .recent-item:hover:not(:disabled) {
        background: var(--hover-bg, #444);
    }
    .recent-item:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
    .actions {
        display: flex;
        gap: 4px;
        margin-top: 8px;
    }
    .actions button {
        flex: 1;
        padding: 4px 8px;
        font-size: 12px;
        border: 1px solid var(--border-color, #555);
        background: var(--bg-secondary, #333);
        color: inherit;
        cursor: pointer;
        border-radius: 4px;
    }
    .actions button:hover {
        background: var(--hover-bg, #444);
    }
    .form {
        display: flex;
        flex-direction: column;
        gap: 4px;
        margin-top: 6px;
    }
    .form input {
        padding: 4px 8px;
        font-size: 12px;
        border: 1px solid var(--border-color, #555);
        background: var(--bg-primary, #1e1e1e);
        color: inherit;
        border-radius: 4px;
    }
    .path-row {
        display: flex;
        gap: 4px;
    }
    .path-row input {
        flex: 1;
    }
    .browse-btn {
        padding: 4px 8px;
        font-size: 12px;
        border: 1px solid var(--border-color, #555);
        background: var(--bg-secondary, #333);
        color: inherit;
        cursor: pointer;
        border-radius: 4px;
    }
    .browse-btn:hover {
        background: var(--hover-bg, #444);
    }
    .form > button {
        padding: 4px 8px;
        font-size: 12px;
        border: none;
        background: var(--accent, #61afef);
        color: #000;
        cursor: pointer;
        border-radius: 4px;
    }
    .form > button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
