import type { NoteMeta } from '$lib/types';

/// Svelte 5 runes store for note content and tree state.
/// Manages the currently active note, its content, dirty flag, and the complete note tree.
const state = $state({
    noteTree: [] as NoteMeta[],
    activeNotePath: null as string | null,
    activeNoteContent: '',
    isDirty: false
});

export function getNoteTree() { return state.noteTree; }
export function getActiveNotePath() { return state.activeNotePath; }
export function getActiveNoteContent() { return state.activeNoteContent; }
export function isDirty() { return state.isDirty; }

export function setTree(notes: NoteMeta[]) {
    state.noteTree = notes;
}

export function setActiveNote(path: string | null) {
    state.activeNotePath = path;
}

export function setContent(content: string) {
    state.activeNoteContent = content;
    state.isDirty = true;
}

export function markClean() {
    state.isDirty = false;
}
