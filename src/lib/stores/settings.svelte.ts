import type { EditorMode } from '$lib/types';

export interface AppSettings {
    theme: 'dark' | 'light';
    defaultEditorMode: EditorMode;
    sidebarVisible: boolean;
    fontSize: number;
    autosaveIntervalMs: number;
}

const defaults: AppSettings = {
    theme: 'dark',
    defaultEditorMode: 'edit',
    sidebarVisible: true,
    fontSize: 16,
    autosaveIntervalMs: 500
};

const state = $state<AppSettings>({ ...defaults });

export function getSettings() { return state; }

export function update(partial: Partial<AppSettings>) {
    Object.assign(state, partial);
}

export function reset() {
    Object.assign(state, defaults);
}
