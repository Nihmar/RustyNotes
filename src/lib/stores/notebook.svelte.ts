import type { Notebook, NotebookInfo } from '$lib/types';

/// Svelte 5 runes store for notebook-related state.
/// Tracks the active notebook, recently opened notebooks list, and loading indicator.
const state = $state({
    activeNotebook: null as Notebook | null,
    recentNotebooks: [] as NotebookInfo[],
    loading: false
});

export function getActiveNotebook() { return state.activeNotebook; }
export function getRecentNotebooks() { return state.recentNotebooks; }
export function isLoading() { return state.loading; }

export function setActive(notebook: Notebook | null) {
    state.activeNotebook = notebook;
}

export function setRecent(list: NotebookInfo[]) {
    state.recentNotebooks = list;
}

export function setLoading(v: boolean) {
    state.loading = v;
}
