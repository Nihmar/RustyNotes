/// Plugin loading and lifecycle management.
///
/// Plugins can be registered programmatically via `registerPlugin()`.
/// Future: dynamic loading from `.rustynotes/plugins/` directory.
///
/// Lifecycle hooks (`applyOnNoteOpen`, `applyOnNoteSave`) are called
/// as a pipeline — each plugin's result feeds into the next.

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

/// Runs all registered plugins' `onNoteOpen` hooks in sequence.
export function applyOnNoteOpen(content: string): string {
    let result = content;
    for (const plugin of plugins) {
        if (plugin.onNoteOpen) {
            result = plugin.onNoteOpen(result);
        }
    }
    return result;
}

/// Runs all registered plugins' `onNoteSave` hooks in sequence.
export function applyOnNoteSave(content: string): string {
    let result = content;
    for (const plugin of plugins) {
        if (plugin.onNoteSave) {
            result = plugin.onNoteSave(result);
        }
    }
    return result;
}
