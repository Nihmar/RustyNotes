import type { EditorMode } from '$lib/types';

let sidebarVisible = $state(true);
let editorMode = $state<EditorMode>('edit');
let theme = $state<'dark' | 'light'>('dark');

export function toggleSidebar() {
    sidebarVisible = !sidebarVisible;
}

export function setSidebarVisible(v: boolean) {
    sidebarVisible = v;
}

export function setEditorMode(mode: EditorMode) {
    editorMode = mode;
}

export function setTheme(t: 'dark' | 'light') {
    theme = t;
}

export function cycleEditorMode() {
    const modes: EditorMode[] = ['edit', 'live-preview', 'reading'];
    const idx = modes.indexOf(editorMode);
    editorMode = modes[(idx + 1) % modes.length];
}

export { sidebarVisible, editorMode, theme };
