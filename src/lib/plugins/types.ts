import type { EditorView } from '@codemirror/view';
import type { Extension } from '@codemirror/state';

export interface Plugin {
    name: string;
    onNoteOpen?: (content: string) => string;
    onNoteSave?: (content: string) => string;
    onEditorInit?: (view: EditorView) => void;
    markdownExtensions?: () => Extension[];
}
