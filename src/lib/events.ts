import { listen } from '@tauri-apps/api/event';
import { setTree } from '$lib/stores/notes.svelte';
import { listNotes } from '$lib/commands';

export async function setupFileWatcher() {
    await listen<{ path: string; event: string }>('note-created', async () => {
        const notes = await listNotes();
        setTree(notes);
    });

    await listen<{ path: string; event: string }>('note-modified', async (event) => {
        const notes = await listNotes();
        setTree(notes);
    });

    await listen<{ path: string; event: string }>('note-deleted', async () => {
        const notes = await listNotes();
        setTree(notes);
    });
}
