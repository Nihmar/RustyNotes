import type { EditorMode } from '$lib/types';

const state = $state({
    sidebarVisible: true,
    editorMode: 'edit' as EditorMode,
    theme: 'dark' as 'dark' | 'light'
});

export function getSidebarVisible() { return state.sidebarVisible; }
export function getEditorMode() { return state.editorMode; }
export function getTheme() { return state.theme; }

export function toggleSidebar() {
    state.sidebarVisible = !state.sidebarVisible;
}

export function setSidebarVisible(v: boolean) {
    state.sidebarVisible = v;
}

export function setEditorMode(mode: EditorMode) {
    state.editorMode = mode;
}

export function setTheme(t: 'dark' | 'light') {
    state.theme = t;
}

export function toggleTheme() {
    state.theme = state.theme === 'dark' ? 'light' : 'dark';
}

export function cycleEditorMode() {
    const modes: EditorMode[] = ['edit', 'live-preview', 'reading'];
    const idx = modes.indexOf(state.editorMode);
    state.editorMode = modes[(idx + 1) % modes.length];
}
