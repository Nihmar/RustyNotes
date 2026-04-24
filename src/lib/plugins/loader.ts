import type { Plugin } from './types';

const plugins: Plugin[] = [];

export async function loadPlugins() {
    // Placeholder: in the future, scan .rustynotes/plugins/ for .js files
    // and dynamically import them
    return plugins;
}

export function registerPlugin(plugin: Plugin) {
    plugins.push(plugin);
}

export function getPlugins(): Plugin[] {
    return plugins;
}

export function applyOnNoteOpen(content: string): string {
    let result = content;
    for (const plugin of plugins) {
        if (plugin.onNoteOpen) {
            result = plugin.onNoteOpen(result);
        }
    }
    return result;
}

export function applyOnNoteSave(content: string): string {
    let result = content;
    for (const plugin of plugins) {
        if (plugin.onNoteSave) {
            result = plugin.onNoteSave(result);
        }
    }
    return result;
}
