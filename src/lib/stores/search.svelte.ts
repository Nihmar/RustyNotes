import type { SearchResult } from '$lib/types';

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
