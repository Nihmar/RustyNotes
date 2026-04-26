<script lang="ts">
    /// Welcome screen — shown when no notebook is active.
    /// Provides buttons to open an existing notebook folder or create a new one,
    /// plus a list of recently opened notebooks for quick access.
    import { onMount } from 'svelte';
    import { getRecentNotebooks, isLoading, setActive, setLoading, setRecent } from '$lib/stores/notebook.svelte';
    import { createNotebook, openNotebook, listNotebooks } from '$lib/commands';
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

    async function handlePickFolder() {
        try {
            const selected = await openDialog({ directory: true, multiple: false });
            if (selected) {
                await openNotebookPath(selected);
            }
        } catch (e: any) {
            error = e;
        }
    }

    async function openNotebookPath(path: string) {
        try {
            setLoading(true);
            error = '';
            const notebook = await openNotebook(path);
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

<div class="welcome">
    <h1>RustyNotes</h1>
    <p>A note-taking app for your local markdown files.</p>

    {#if error}
        <p class="error">{error}</p>
    {/if}

    <div class="welcome-actions">
        <button class="primary" onclick={handlePickFolder}>
            Open Notebook
        </button>
        <button onclick={() => (showNewForm = !showNewForm)}>
            New Notebook
        </button>
    </div>

    {#if getRecentNotebooks().length > 0}
        <div class="recent-section">
            <h2>Recent Notebooks</h2>
            <div class="recent-list">
                {#each getRecentNotebooks() as nb}
                    <button class="recent-item" onclick={() => handleOpenRecent(nb)} disabled={isLoading()}>
                        <span class="ri-name">{nb.name}</span>
                        <span class="ri-path">{nb.path}</span>
                    </button>
                {/each}
            </div>
        </div>
    {/if}

    {#if showNewForm}
        <form class="welcome-form" onsubmit={(e) => { e.preventDefault(); handleCreate(); }}>
            <input type="text" placeholder="Notebook name..." bind:value={newName} />
            <div class="path-row">
                <input type="text" placeholder="Path to create notebook..." bind:value={newPath} />
                <button type="button" class="browse-btn" onclick={handlePickNewFolder}>...</button>
            </div>
            <button type="submit" disabled={isLoading()}>Create</button>
        </form>
    {/if}
</div>

<style>
    .welcome {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 100vh;
        gap: 12px;
        text-align: center;
        padding: 24px;
        overflow-y: auto;
    }
    h1 {
        margin: 0;
        font-size: 28px;
        color: var(--accent, #61afef);
    }
    p {
        color: var(--text-muted, #888);
        margin: 0;
    }
    .error {
        color: #e06c75;
        font-size: 13px;
        margin: 0;
    }
    .welcome-actions {
        display: flex;
        gap: 8px;
        margin-top: 16px;
    }
    .welcome-actions button {
        padding: 8px 16px;
        font-size: 14px;
        border: 1px solid var(--border-color, #555);
        background: var(--bg-secondary, #333);
        color: inherit;
        cursor: pointer;
        border-radius: 6px;
    }
    .welcome-actions button.primary {
        background: var(--accent, #61afef);
        color: #000;
        border-color: var(--accent, #61afef);
    }
    .welcome-actions button:hover {
        opacity: 0.85;
    }
    .recent-section {
        margin-top: 24px;
        width: 100%;
        max-width: 420px;
    }
    .recent-section h2 {
        font-size: 13px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: var(--text-muted, #888);
        margin-bottom: 8px;
        text-align: left;
    }
    .recent-list {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }
    .recent-item {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        width: 100%;
        padding: 10px 12px;
        background: var(--bg-secondary, #2a2a2a);
        border: 1px solid var(--border-color, #444);
        border-radius: 6px;
        color: inherit;
        cursor: pointer;
        text-align: left;
        transition: background 0.15s;
    }
    .recent-item:hover:not(:disabled) {
        background: var(--hover-bg, #3a3a3a);
    }
    .recent-item:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
    .ri-name {
        font-size: 14px;
        font-weight: 600;
    }
    .ri-path {
        font-size: 11px;
        color: var(--text-muted, #888);
        margin-top: 2px;
    }
    .welcome-form {
        display: flex;
        flex-direction: column;
        gap: 8px;
        width: 320px;
        margin-top: 8px;
    }
    .welcome-form input {
        padding: 8px 12px;
        font-size: 13px;
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
        padding: 8px 10px;
        font-size: 14px;
        border: 1px solid var(--border-color, #555);
        background: var(--bg-secondary, #333);
        color: inherit;
        cursor: pointer;
        border-radius: 4px;
    }
    .browse-btn:hover {
        background: var(--hover-bg, #444);
    }
    .welcome-form > button {
        padding: 8px 16px;
        font-size: 13px;
        border: none;
        background: var(--accent, #61afef);
        color: #000;
        cursor: pointer;
        border-radius: 4px;
    }
    .welcome-form > button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
