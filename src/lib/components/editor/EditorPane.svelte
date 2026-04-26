<script lang="ts">
    /// Editor pane — the main content area component.
    /// Orchestrates tab management, mode switching, autosave, and theme toggling.
    /// Bridges the editor store with the Editor component and the Rust backend.
    import { untrack } from 'svelte';
    import Editor from './Editor.svelte';
    import ModeSwitcher from './ModeSwitcher.svelte';
    import TabBar from './TabBar.svelte';
    import { getActiveTab, getActiveTabIndex, openTab } from '$lib/stores/tabs.svelte';
    import { getActiveNoteContent, getActiveNotePath, setActiveNote, setContent, isDirty, markClean } from '$lib/stores/notes.svelte';
    import { writeNote, readNote, createNote } from '$lib/commands';
    import { getActiveNotebook } from '$lib/stores/notebook.svelte';
    import { getEditorMode, getTheme, toggleTheme } from '$lib/stores/ui.svelte';

    let editorRef: Editor | undefined = $state();
    let saveTimer: ReturnType<typeof setTimeout> | null = null;
    let initialContent = $state('');

    function handleContentChange() {
        if (!editorRef) return;
        const content = editorRef.getContent();
        setContent(content);

        if (saveTimer) clearTimeout(saveTimer);
        saveTimer = setTimeout(async () => {
            const activePath = getActiveNotePath();
            if (!activePath) return;
            try {
                await writeNote(activePath, content);
                markClean();
            } catch (e) {
                console.error('Autosave failed:', e);
            }
        }, 500);
    }

    // When a note is selected from the sidebar, open its tab
    $effect(() => {
        const path = getActiveNotePath();
        const content = getActiveNoteContent();
        if (path && content !== undefined && content !== null) {
            untrack(() => {
                const title = path.split(/[/\\]/).pop()?.replace('.md', '') ?? 'Untitled';
                openTab({
                    path,
                    title,
                    isDirty: isDirty(),
                    mode: getEditorMode()
                });
            });
            initialContent = content;
        }
    });

    // When active tab changes (e.g. clicked tab in TabBar), load the note content
    $effect(() => {
        const tab = getActiveTab();
        const _idx = getActiveTabIndex();
        if (!tab) {
            initialContent = '';
            setActiveNote(null);
            setContent('');
            markClean();
            return;
        }
        // If sidebar already loaded this note, use its content directly
        if (tab.path === getActiveNotePath() && getActiveNoteContent()) {
            const existing = getActiveNoteContent();
            if (initialContent !== existing) {
                initialContent = existing;
            }
            return;
        }
        // Load from disk for direct tab switches
        const tabPath = tab.path;
        readNote(tabPath).then((content) => {
            if (getActiveTab()?.path !== tabPath) return;
            setActiveNote(tabPath);
            setContent(content);
            markClean();
            initialContent = content;
        }).catch((e) => {
            console.error('Failed to load note for tab:', e);
        });
    });

    // Propagate theme changes to the editor ref
    $effect(() => {
        const currentTheme = getTheme();
        editorRef?.setEditorTheme(currentTheme);
    });

    async function handleCreateNote() {
        if (!getActiveNotebook()) return;
        const title = 'Untitled';
        try {
            const meta = await createNote('.', title);
            setActiveNote(meta.path);
            initialContent = '';
        } catch (e) {
            console.error('Failed to create note:', e);
        }
    }
</script>

<div class="editor-pane">
    {#if getActiveTab()}
        <div class="toolbar">
            <TabBar />
            <ModeSwitcher />
            <button class="theme-btn" onclick={toggleTheme} title="Toggle theme">
                {getTheme() === 'dark' ? '☀' : '☾'}
            </button>
            <span class="save-status">
                {isDirty() ? 'Unsaved' : 'Saved'}
            </span>
        </div>
        <Editor
            bind:this={editorRef}
            content={initialContent}
            mode={getEditorMode()}
            theme={getTheme()}
            onchange={handleContentChange}
        />
    {:else}
        <div class="no-note">
            <p>No notes open</p>
            {#if getActiveNotebook()}
                <button class="create-btn" onclick={handleCreateNote}>
                    + Create new note
                </button>
            {/if}
        </div>
    {/if}
</div>

<style>
    .editor-pane {
        flex: 1;
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: hidden;
    }
    .toolbar {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 4px 8px;
        border-bottom: 1px solid var(--border-color, #333);
        background: var(--bg-secondary, #252525);
    }
    .save-status {
        font-size: 11px;
        color: var(--text-muted, #888);
        margin-left: auto;
    }
    .theme-btn {
        border: none;
        background: transparent;
        font-size: 16px;
        cursor: pointer;
        padding: 2px 6px;
        border-radius: 4px;
        line-height: 1;
    }
    .theme-btn:hover {
        background: var(--bg-secondary, #2a2a2a);
    }
    .no-note {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 12px;
        color: var(--text-muted, #666);
    }
    .create-btn {
        padding: 8px 20px;
        font-size: 14px;
        border: 1px solid var(--accent, #61afef);
        background: transparent;
        color: var(--accent, #61afef);
        cursor: pointer;
        border-radius: 6px;
    }
    .create-btn:hover {
        background: var(--accent, #61afef);
        color: #000;
    }
</style>
