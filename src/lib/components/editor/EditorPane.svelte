<script lang="ts">
    import Editor from './Editor.svelte';
    import ModeSwitcher from './ModeSwitcher.svelte';
    import TabBar from './TabBar.svelte';
    import { getActiveTab, openTab } from '$lib/stores/tabs.svelte';
    import { getActiveNoteContent, getActiveNotePath, setContent, isDirty, markClean } from '$lib/stores/notes.svelte';
    import { writeNote } from '$lib/commands';

    let editorRef: Editor | undefined = $state();
    let saveTimer: ReturnType<typeof setTimeout> | null = null;

    function handleContentChange() {
        if (!editorRef) return;
        const content = editorRef.getContent();
        setContent(content);

        // Autosave debounced
        if (saveTimer) clearTimeout(saveTimer);
        saveTimer = setTimeout(async () => {
            if (!getActiveNotePath()) return;
            try {
                await writeNote(getActiveNotePath(), content);
                markClean();
            } catch (e) {
                console.error('Autosave failed:', e);
            }
        }, 500);
    }

    $effect(() => {
        const path = getActiveNotePath();
        const content = getActiveNoteContent();
        if (path && content !== undefined && content !== null) {
            const title = path.split(/[/\\]/).pop()?.replace('.md', '') ?? 'Untitled';
            openTab({
                path,
                title,
                isDirty: isDirty(),
                mode: 'edit'
            });
        }
    });
</script>

<div class="editor-pane">
    {#if getActiveTab()}
        <div class="toolbar">
            <TabBar />
            <ModeSwitcher />
            <span class="save-status">
                {isDirty() ? 'Unsaved' : 'Saved'}
            </span>
        </div>
        <Editor
            bind:this={editorRef}
            content={getActiveNoteContent()}
        />
    {:else}
        <div class="no-note">
            <p>Select a note from the sidebar or create a new one</p>
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
    .no-note {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
        color: var(--text-muted, #666);
    }
</style>
