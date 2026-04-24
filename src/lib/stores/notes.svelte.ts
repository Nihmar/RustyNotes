import type { NoteMeta } from '$lib/types';

let noteTree = $state<NoteMeta[]>([]);
let activeNotePath = $state<string | null>(null);
let activeNoteContent = $state('');
let isDirty = $state(false);

export function setTree(notes: NoteMeta[]) {
    noteTree = notes;
}

export function setActiveNote(path: string | null) {
    activeNotePath = path;
}

export function setContent(content: string) {
    activeNoteContent = content;
    isDirty = true;
}

export function markClean() {
    isDirty = false;
}

export { noteTree, activeNotePath, activeNoteContent, isDirty };
