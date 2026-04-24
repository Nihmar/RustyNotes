<script lang="ts">
    import { onMount } from 'svelte';
    import { listNotes, readNote, deleteNote, renameNote, createNote } from '$lib/commands';
    import { buildFileTree } from '$lib/commands';
    import {
        getActiveNotePath,
        setTree, setActiveNote, setContent, markClean
    } from '$lib/stores/notes.svelte';
    import { getActiveNotebook } from '$lib/stores/notebook.svelte';
    import type { TreeNode } from '$lib/types';

    let fileTree = $state<TreeNode[]>([]);
    let expanded = $state<Set<string>>(new Set());
    let contextMenu = $state<{ node: TreeNode; x: number; y: number } | null>(null);
    let renaming = $state<TreeNode | null>(null);
    let renameValue = $state('');
    let creating = $state<string | null>(null);
    let createValue = $state('');

    async function loadNotes() {
        try {
            const notes = await listNotes();
            setTree(notes);
            fileTree = buildFileTree(notes);
        } catch (e) {
            console.error('Failed to load notes:', e);
        }
    }

    onMount(() => {
        loadNotes();
    });

    $effect(() => {
        void getActiveNotebook();
        loadNotes();
    });

    function toggleFolder(path: string) {
        const next = new Set(expanded);
        if (next.has(path)) {
            next.delete(path);
        } else {
            next.add(path);
        }
        expanded = next;
    }

    async function handleOpenNote(node: TreeNode) {
        if (node.type !== 'file' || !node.meta) return;
        try {
            const content = await readNote(node.meta.path);
            setActiveNote(node.meta.path);
            setContent(content);
            markClean();
        } catch (e) {
            console.error('Failed to open note:', e);
        }
    }

    function onContextMenu(e: MouseEvent, node: TreeNode) {
        e.preventDefault();
        e.stopPropagation();
        contextMenu = { node, x: e.clientX, y: e.clientY };
    }

    function startRename(node: TreeNode) {
        renaming = node;
        renameValue = node.name;
        contextMenu = null;
    }

    async function submitRename() {
        if (!renaming || !renameValue.trim()) {
            renaming = null;
            return;
        }
        try {
            await renameNote(renaming.path, renameValue.trim());
            renaming = null;
            await loadNotes();
        } catch (e) {
            console.error('Failed to rename:', e);
        }
    }

    async function handleDelete(node: TreeNode) {
        contextMenu = null;
        if (!confirm(`Delete "${node.name}"?`)) return;
        try {
            await deleteNote(node.path);
            if (getActiveNotePath() === node.path) {
                setActiveNote(null);
                setContent('');
            }
            await loadNotes();
        } catch (e) {
            console.error('Failed to delete:', e);
        }
    }

    function startCreate(parentPath: string) {
        creating = parentPath;
        createValue = '';
        contextMenu = null;
    }

    async function submitCreate() {
        const parentPath = creating;
        if (!parentPath || !createValue.trim()) {
            creating = null;
            return;
        }
        try {
            await createNote(parentPath, createValue.trim());
            creating = null;
            const next = new Set(expanded);
            next.add(parentPath);
            expanded = next;
            await loadNotes();
        } catch (e) {
            console.error('Failed to create note:', e);
        }
    }

    function isActiveNode(node: TreeNode): boolean {
        return node.type === 'file' && getActiveNotePath() === node.path;
    }

    $effect(() => {
        if (!contextMenu) return;
        function handler() {
            contextMenu = null;
        }
        window.addEventListener('click', handler);
        return () => window.removeEventListener('click', handler);
    });

    $effect(() => {
        if (!contextMenu && !renaming && !creating) return;
        function handler(e: KeyboardEvent) {
            if (e.key === 'Escape') {
                contextMenu = null;
                renaming = null;
                creating = null;
            }
        }
        window.addEventListener('keydown', handler);
        return () => window.removeEventListener('keydown', handler);
    });
</script>

<button class="new-note-btn" onclick={() => startCreate('.')}>
    + New Note
</button>

<div class="filetree">
    {#if fileTree.length === 0}
        <p class="empty">No notes yet</p>
    {/if}
    {#each fileTree as node}
        {@render treeNode(node, 0)}
    {/each}
</div>

{#snippet treeNode(node: TreeNode, depth: number)}
    {@const active = isActiveNode(node)}
    <div class="tree-row" style="padding-left: {depth * 16 + 4}px">
        {#if node.type === 'folder'}
            <button
                class="folder-toggle"
                onclick={() => toggleFolder(node.path)}
                aria-label={expanded.has(node.path) ? 'Collapse folder' : 'Expand folder'}
            >
                {expanded.has(node.path) ? '▾' : '▸'}
            </button>
            <span class="folder-name"
                onclick={() => toggleFolder(node.path)}
                onkeydown={(e) => e.key === 'Enter' && toggleFolder(node.path)}
                role="button"
                tabindex="0"
            >{node.name}</span>
        {:else}
            <span class="file-icon">📄</span>
            <button
                class="file-name"
                class:active
                onclick={() => handleOpenNote(node)}
                oncontextmenu={(e) => onContextMenu(e, node)}
            >
                {node.name}
            </button>
        {/if}
    </div>
    {#if node.type === 'folder' && expanded.has(node.path)}
        {#each node.children as child}
            {@render treeNode(child, depth + 1)}
        {/each}
    {/if}
{/snippet}

{#if contextMenu}
    {@const ctx = contextMenu}
    <div
        class="context-menu"
        style="left: {ctx.x}px; top: {ctx.y}px;"
        role="menu"
    >
        {#if ctx.node.type === 'file'}
            <button role="menuitem" onclick={() => handleOpenNote(ctx.node)}>Open</button>
            <button role="menuitem" onclick={() => startRename(ctx.node)}>Rename</button>
            <button role="menuitem" class="danger" onclick={() => handleDelete(ctx.node)}>Delete</button>
        {:else}
            <button role="menuitem" onclick={() => startCreate(ctx.node.path)}>New Note</button>
        {/if}
    </div>
{/if}

{#if renaming}
    <div class="modal-overlay" role="dialog" aria-label="Rename note" onclick={() => (renaming = null)} onkeydown={() => {}}>
        <div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={() => {}} role="document">
            <form onsubmit={(e) => { e.preventDefault(); submitRename(); }}>
                <label for="rename-input">Rename</label>
                <input id="rename-input" type="text" bind:value={renameValue} autofocus />
                <div class="modal-actions">
                    <button type="submit">OK</button>
                    <button type="button" onclick={() => (renaming = null)}>Cancel</button>
                </div>
            </form>
        </div>
    </div>
{/if}

{#if creating}
    <div class="modal-overlay" role="dialog" aria-label="Create note" onclick={() => (creating = null)} onkeydown={() => {}}>
        <div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={() => {}} role="document">
            <form onsubmit={(e) => { e.preventDefault(); submitCreate(); }}>
                <label for="create-input">New Note</label>
                <input
                    id="create-input"
                    type="text"
                    placeholder="Note title..."
                    bind:value={createValue}
                    autofocus
                />
                <div class="modal-actions">
                    <button type="submit">Create</button>
                    <button type="button" onclick={() => (creating = null)}>Cancel</button>
                </div>
            </form>
        </div>
    </div>
{/if}

<style>
    .filetree {
        overflow-y: auto;
        flex: 1;
    }
    .new-note-btn {
        width: 100%;
        padding: 6px 8px;
        font-size: 12px;
        border: none;
        background: var(--bg-secondary, #333);
        color: var(--text-muted, #888);
        cursor: pointer;
        text-align: left;
    }
    .new-note-btn:hover {
        background: var(--hover-bg, #444);
    }
    .empty {
        padding: 8px;
        font-size: 12px;
        color: var(--text-muted, #666);
        text-align: center;
    }
    .tree-row {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 2px 8px;
        font-size: 13px;
        min-height: 26px;
    }
    .tree-row:hover {
        background: var(--hover-bg, #2a2a2a);
    }
    .folder-toggle {
        background: none;
        border: none;
        color: var(--text-muted, #888);
        cursor: pointer;
        padding: 0 2px;
        font-size: 10px;
        width: 16px;
        text-align: center;
    }
    .folder-name {
        color: var(--text-muted, #aaa);
        font-size: 13px;
        cursor: pointer;
        outline: none;
    }
    .file-icon {
        font-size: 12px;
        opacity: 0.7;
    }
    .file-name {
        background: none;
        border: none;
        color: inherit;
        cursor: pointer;
        font-size: 13px;
        padding: 0;
        text-align: left;
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
    .file-name.active {
        color: var(--accent, #61afef);
    }
    .context-menu {
        position: fixed;
        background: var(--bg-primary, #1e1e1e);
        border: 1px solid var(--border-color, #444);
        border-radius: 6px;
        padding: 4px;
        z-index: 1000;
        min-width: 120px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    }
    .context-menu button {
        display: block;
        width: 100%;
        background: none;
        border: none;
        color: inherit;
        cursor: pointer;
        padding: 6px 12px;
        font-size: 12px;
        text-align: left;
        border-radius: 4px;
    }
    .context-menu button:hover {
        background: var(--hover-bg, #444);
    }
    .context-menu button.danger {
        color: #e06c75;
    }
    .modal-overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1001;
    }
    .modal {
        background: var(--bg-primary, #1e1e1e);
        border: 1px solid var(--border-color, #444);
        border-radius: 8px;
        padding: 16px;
        min-width: 280px;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
    }
    .modal label {
        display: block;
        font-size: 13px;
        margin-bottom: 8px;
        color: var(--text-muted, #888);
    }
    .modal input {
        width: 100%;
        padding: 6px 10px;
        font-size: 13px;
        border: 1px solid var(--border-color, #555);
        background: var(--bg-secondary, #2a2a2a);
        color: inherit;
        border-radius: 4px;
        box-sizing: border-box;
    }
    .modal-actions {
        display: flex;
        gap: 8px;
        margin-top: 12px;
        justify-content: flex-end;
    }
    .modal-actions button {
        padding: 6px 14px;
        font-size: 12px;
        border: 1px solid var(--border-color, #555);
        border-radius: 4px;
        cursor: pointer;
        background: var(--bg-secondary, #333);
        color: inherit;
    }
    .modal-actions button[type="submit"] {
        background: var(--accent, #61afef);
        color: #000;
        border-color: var(--accent, #61afef);
    }
</style>
