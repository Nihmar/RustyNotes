import type { Notebook, NotebookInfo } from '$lib/types';

let activeNotebook = $state<Notebook | null>(null);
let recentNotebooks = $state<NotebookInfo[]>([]);
let loading = $state(false);

export function setActive(notebook: Notebook | null) {
    activeNotebook = notebook;
}

export function setRecent(list: NotebookInfo[]) {
    recentNotebooks = list;
}

export function setLoading(v: boolean) {
    loading = v;
}

export { activeNotebook, recentNotebooks, loading };
