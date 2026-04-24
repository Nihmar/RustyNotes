export interface NotebookInfo {
    name: string;
    path: string;
}

export interface Notebook {
    name: string;
    path: string;
    created: string;
}

export interface NoteMeta {
    title: string;
    path: string;
    modified: string;
    size: number;
    tags: string[];
}

export interface SearchResult {
    path: string;
    title: string;
    snippet: string;
    line: number;
    relevance: number;
}

export interface TagInfo {
    name: string;
    count: number;
}

export interface TreeNode {
    name: string;
    path: string;
    type: 'folder' | 'file';
    children: TreeNode[];
    meta?: NoteMeta;
}

export type EditorMode = 'edit' | 'live-preview' | 'reading';

export interface Plugin {
    name: string;
    onNoteOpen?: (content: string) => string;
    onNoteSave?: (content: string) => string;
    onEditorInit?: (view: unknown) => void;
    markdownExtensions?: () => unknown[];
}
