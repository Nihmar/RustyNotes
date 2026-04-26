/// Wiki-link support for the CodeMirror editor.
///
/// Provides:
/// - Syntax highlighting for `[[wiki-link]]` patterns in the editor
/// - Parsing (`parseWikilink`) and navigation (`navigateWikilink`)
/// - Automatic note creation when navigating to non-existent wiki-link targets

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
    const regex = /(?<!!)\[\[([^\]]+)\]\]/g;
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
import { openNewTab } from '$lib/stores/tabs.svelte';
import { setActiveNote, setContent, markClean, getNoteTree } from '$lib/stores/notes.svelte';
import { getEditorMode } from '$lib/stores/ui.svelte';
import type { NoteMeta } from '$lib/types';

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

function findNoteByTitle(notes: NoteMeta[], title: string): NoteMeta | null {
    const targetLower = title.toLowerCase();
    for (const note of notes) {
        if (note.title.toLowerCase() === targetLower) {
            return note;
        }
    }
    return null;
}

export async function navigateWikilink(rawText: string, newTab: boolean) {
    const { target } = parseWikilink(rawText);
    const title = wikilinkTitle(target);
    const mode = getEditorMode();

    const notes = getNoteTree();
    const match = findNoteByTitle(notes, title);

    const targetPath = match ? match.path : wikilinkToPath(target);
    const finalTitle = match ? match.title : title;

    let content: string;
    try {
        content = await readNote(targetPath);
    } catch {
        const parentDir = targetPath.includes('/')
            ? targetPath.substring(0, targetPath.lastIndexOf('/'))
            : '.';
        const meta = await createNote(parentDir, finalTitle);
        content = `# ${meta.title}\n\n`;
    }

    // Update the notes store — Effect 1 (EditorPane) will pick this up and open a tab.
    // This mirrors exactly how FileTree.handleOpenNote works.
    setActiveNote(targetPath);
    setContent(content);
    markClean();

    // For ctrl+click / cmd+click: also open a dedicated new tab.
    // Effect 1 will still call openTab on the same path, which becomes a no-op
    // since openNewTab already created the tab.
    if (newTab) {
        openNewTab({ path: targetPath, title: finalTitle, isDirty: false, mode });
    }
}
