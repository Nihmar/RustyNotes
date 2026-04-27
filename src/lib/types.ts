/// Lightweight notebook reference (no creation timestamp), used in recent notebooks list.
export interface NotebookInfo {
    name: string;
    path: string;
}

/// Full notebook metadata including creation timestamp.
export interface Notebook {
    name: string;
    path: string;
    created: string;
}

/// Metadata for a single markdown note, returned from the Rust backend.
export interface NoteMeta {
    title: string;
    path: string;
    modified: string;
    size: number;
    tags: string[];
}

/// A single search result with context snippet and relevance score.
export interface SearchResult {
    path: string;
    title: string;
    snippet: string;
    line: number;
    relevance: number;
}

/// A tag name and the number of notes it appears in.
export interface TagInfo {
    name: string;
    count: number;
}

/// A node in the file tree sidebar — can be a folder or a file (note).
export interface TreeNode {
    name: string;
    path: string;
    type: 'folder' | 'file';
    children: TreeNode[];
    meta?: NoteMeta;
}

/// Editor display modes.
export type EditorMode = 'edit' | 'reading';

/// Plugin interface for extending editor behavior (content transforms, editor init hooks, markdown extensions).
export interface Plugin {
    name: string;
    onNoteOpen?: (content: string) => string;
    onNoteSave?: (content: string) => string;
    onEditorInit?: (view: unknown) => void;
    markdownExtensions?: () => unknown[];
}
