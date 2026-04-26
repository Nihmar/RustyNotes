import {
    Decoration,
    EditorView,
    ViewPlugin,
    ViewUpdate
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';
import { syntaxTree } from '@codemirror/language';

const wikiLinkDeco = Decoration.mark({ class: 'cm-wikilink' });

function decorateWikilinks(view: EditorView): DecorationSet {
    const builder = new RangeSetBuilder<Decoration>();
    const doc = view.state.doc;

    // Simple regex approach for wikilinks
    const text = doc.toString();
    const regex = /\[\[([^\]]+)\]\]/g;
    let match;

    while ((match = regex.exec(text)) !== null) {
        const from = match.index;
        const to = match.index + match[0].length;
        builder.add(from, from + 2, Decoration.mark({ class: 'cm-wikilink-bracket' }));
        builder.add(from + 2, to - 2, Decoration.mark({ class: 'cm-wikilink-content' }));
        builder.add(to - 2, to, Decoration.mark({ class: 'cm-wikilink-bracket' }));
    }

    return builder.finish();
}

const wikilinkPlugin = ViewPlugin.fromClass(class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
        this.decorations = decorateWikilinks(view);
    }

    update(update: ViewUpdate) {
        if (update.docChanged || update.viewportChanged) {
            this.decorations = decorateWikilinks(update.view);
        }
    }
}, {
    decorations: (v) => v.decorations
});

export function wikilinks() {
    return wikilinkPlugin;
}

// ── Navigation ──

import { readNote, createNote } from '$lib/commands';
import { openTab, openNewTab } from '$lib/stores/tabs.svelte';
import { getEditorMode } from '$lib/stores/ui.svelte';

export function parseWikilink(raw: string): { target: string; display: string } {
    const parts = raw.split('|');
    const target = parts[0].trim();
    const display = parts.length > 1 ? parts[1].trim() : target;
    return { target, display };
}

export function wikilinkToPath(target: string): string {
    return target.endsWith('.md') ? target : `${target}.md`;
}

export function wikilinkTitle(target: string): string {
    return target.split('/').pop()?.replace('.md', '') ?? 'Untitled';
}

export async function navigateWikilink(rawText: string, newTab: boolean) {
    const { target } = parseWikilink(rawText);
    const targetPath = wikilinkToPath(target);
    const title = wikilinkTitle(target);
    const mode = getEditorMode();

    try {
        await readNote(targetPath);
    } catch {
        const parentDir = targetPath.includes('/')
            ? targetPath.substring(0, targetPath.lastIndexOf('/'))
            : '.';
        await createNote(parentDir, title);
    }

    if (newTab) {
        openNewTab({ path: targetPath, title, isDirty: false, mode });
    } else {
        openTab({ path: targetPath, title, isDirty: false, mode });
    }
}
