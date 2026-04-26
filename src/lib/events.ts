import { listen } from '@tauri-apps/api/event';
import { setTree, setActiveNote, setContent, markClean } from '$lib/stores/notes.svelte';
import { getActiveNotebook } from '$lib/stores/notebook.svelte';
import { getTabByPath } from '$lib/stores/tabs.svelte';
import { listNotes, readNote } from '$lib/commands';

export async function setupFileWatcher() {
    await listen<{ path: string; event: string }>('note-created', async () => {
        const notes = await listNotes();
        setTree(notes);
    });

    await listen<{ path: string; event: string }>('note-modified', async (event) => {
        const notes = await listNotes();
        setTree(notes);

        const notebook = getActiveNotebook();
        if (!notebook) return;
        const root = notebook.path.replace(/\/?$/, '/');
        if (!event.payload.path.startsWith(root)) return;
        const relative = event.payload.path.slice(root.length);

        const tab = getTabByPath(relative);
        if (!tab) return;

        try {
            const content = await readNote(relative);
            if (tab.isDirty) {
                if (!confirm(`"${tab.title}" was modified externally.\nReload from disk?`)) return;
            }
            setActiveNote(relative);
            setContent(content);
            markClean();
        } catch (e) {
            console.error('Failed to reload modified note:', e);
        }
    });

    await listen<{ path: string; event: string }>('note-deleted', async () => {
        const notes = await listNotes();
        setTree(notes);
    });
}
