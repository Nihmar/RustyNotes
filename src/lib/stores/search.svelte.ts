import type { SearchResult } from '$lib/types';

let query = $state('');
let results = $state<SearchResult[]>([]);
let searching = $state(false);

export function setQuery(q: string) {
    query = q;
}

export function setResults(r: SearchResult[]) {
    results = r;
}

export function setSearching(v: boolean) {
    searching = v;
}

export { query, results, searching };
