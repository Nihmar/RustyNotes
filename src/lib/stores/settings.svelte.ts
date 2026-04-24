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

let settings = $state<AppSettings>({ ...defaults });

export function update(partial: Partial<AppSettings>) {
    settings = { ...settings, ...partial };
}

export function reset() {
    settings = { ...defaults };
}

export { settings };
