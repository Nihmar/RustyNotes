import type { EditorView } from '@codemirror/view';
import type { Extension } from '@codemirror/state';

/// Plugin interface for extending RustyNotes.
///
/// Hooks:
/// - `onNoteOpen` — transform note content when opening
/// - `onNoteSave` — transform note content before saving
/// - `onEditorInit` — called after the CodeMirror editor initializes
/// - `markdownExtensions` — return additional CodeMirror extensions
export interface Plugin {
    name: string;
    onNoteOpen?: (content: string) => string;
    onNoteSave?: (content: string) => string;
    onEditorInit?: (view: EditorView) => void;
    markdownExtensions?: () => Extension[];
}
