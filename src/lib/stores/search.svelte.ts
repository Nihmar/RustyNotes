import type { SearchResult } from '$lib/types';

/// Svelte 5 runes store for search state.
/// Tracks the current search query, results list, and loading indicator.
const state = $state({
    query: '',
    results: [] as SearchResult[],
    searching: false
});

export function getQuery() { return state.query; }
export function getResults() { return state.results; }
export function isSearching() { return state.searching; }

export function setQuery(q: string) {
    state.query = q;
}

export function setResults(r: SearchResult[]) {
    state.results = r;
}

export function setSearching(v: boolean) {
    state.searching = v;
}
