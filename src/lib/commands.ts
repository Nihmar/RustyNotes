import { invoke } from '@tauri-apps/api/core';
import type { Notebook, NotebookInfo, NoteMeta, SearchResult, TagInfo } from './types';

// ── Notebook Commands ──

export async function createNotebook(name: string, path: string): Promise<Notebook> {
    return invoke('create_notebook', { name, path });
}

export async function openNotebook(path: string): Promise<Notebook> {
    return invoke('open_notebook', { path });
}

export async function listNotebooks(): Promise<NotebookInfo[]> {
    return invoke('list_notebooks');
}

export async function closeNotebook(): Promise<void> {
    return invoke('close_notebook');
}

// ── Note Commands ──

export async function listNotes(dir?: string): Promise<NoteMeta[]> {
    return invoke('list_notes', { dir });
}

export async function readNote(path: string): Promise<string> {
    return invoke('read_note', { path });
}

export async function writeNote(path: string, content: string): Promise<void> {
    return invoke('write_note', { path, content });
}

export async function createNote(dir: string, title: string): Promise<NoteMeta> {
    return invoke('create_note', { dir, title });
}

export async function deleteNote(path: string): Promise<void> {
    return invoke('delete_note', { path });
}

export async function renameNote(path: string, newName: string): Promise<void> {
    return invoke('rename_note', { path, newName });
}

export async function moveNote(from: string, to: string): Promise<void> {
    return invoke('move_note', { from, to });
}

// ── Search Commands ──

export async function searchNotes(query: string): Promise<SearchResult[]> {
    return invoke('search_notes', { query });
}

// ── Tags Commands ──

export async function getTags(): Promise<TagInfo[]> {
    return invoke('get_tags');
}

// ── Tree Builder ──

import type { TreeNode } from './types';

export function buildFileTree(notes: NoteMeta[]): TreeNode[] {
    const root: TreeNode[] = [];
    const folderMap = new Map<string, TreeNode>();

    const sorted = [...notes].sort((a, b) => a.path.localeCompare(b.path));

    for (const note of sorted) {
        const parts = note.path.replace(/\\/g, '/').split('/');
        let current = root;
        let currentPath = '';

        for (let i = 0; i < parts.length; i++) {
            const part = parts[i];
            currentPath = currentPath ? `${currentPath}/${part}` : part;
            const isFile = i === parts.length - 1;

            if (isFile) {
                const name = part.replace(/\.md$/, '');
                current.push({
                    name,
                    path: note.path,
                    type: 'file',
                    children: [],
                    meta: note
                });
            } else {
                let folder = folderMap.get(currentPath);
                if (!folder) {
                    folder = {
                        name: part,
                        path: currentPath,
                        type: 'folder',
                        children: []
                    };
                    folderMap.set(currentPath, folder);
                    current.push(folder);
                }
                current = folder.children;
            }
        }
    }

    sortTree(root);
    return root;
}

function sortTree(nodes: TreeNode[]) {
    nodes.sort((a, b) => {
        if (a.type !== b.type) return a.type === 'folder' ? -1 : 1;
        return a.name.toLowerCase().localeCompare(b.name.toLowerCase());
    });
    for (const node of nodes) {
        if (node.type === 'folder') {
            sortTree(node.children);
        }
    }
}
