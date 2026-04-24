<script lang="ts">
    import { getActiveNotebook, isLoading, setActive, setLoading } from '$lib/stores/notebook.svelte';
    import { createNotebook, openNotebook } from '$lib/commands';

    let showNewForm = $state(false);
    let showOpenForm = $state(false);
    let newName = $state('');
    let newPath = $state('');
    let openPath = $state('');
    let error = $state('');

    async function handleOpenFolder() {
        if (!openPath.trim()) return;
        try {
            setLoading(true);
            error = '';
            const notebook = await openNotebook(openPath.trim());
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
        } catch (e: any) {
            error = e;
        } finally {
            setLoading(false);
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
        <button class="primary" onclick={() => (showOpenForm = !showOpenForm)}>
            Open Notebook
        </button>
        <button onclick={() => (showNewForm = !showNewForm)}>
            New Notebook
        </button>
    </div>

    {#if showOpenForm}
        <form class="welcome-form" onsubmit={(e) => { e.preventDefault(); handleOpenFolder(); }}>
            <input type="text" placeholder="Path to notebook folder..." bind:value={openPath} />
            <button type="submit" disabled={isLoading()}>Open</button>
        </form>
    {/if}

    {#if showNewForm}
        <form class="welcome-form" onsubmit={(e) => { e.preventDefault(); handleCreate(); }}>
            <input type="text" placeholder="Notebook name..." bind:value={newName} />
            <input type="text" placeholder="Path to create notebook..." bind:value={newPath} />
            <button type="submit" disabled={isLoading()}>Create</button>
        </form>
    {/if}
</div>

<style>
    .welcome {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        gap: 12px;
        text-align: center;
        padding: 24px;
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
    .welcome-form button {
        padding: 8px 16px;
        font-size: 13px;
        border: none;
        background: var(--accent, #61afef);
        color: #000;
        cursor: pointer;
        border-radius: 4px;
    }
    .welcome-form button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
