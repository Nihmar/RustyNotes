<script lang="ts">
    /// Search panel — debounced full-text search across all notes.
    /// Displays results with highlighted snippets and opens notes on click.
    import { searchNotes, readNote } from '$lib/commands';
    import { getQuery, getResults, isSearching, setQuery, setResults, setSearching } from '$lib/stores/search.svelte';
    import { setActiveNote, setContent, markClean } from '$lib/stores/notes.svelte';
    import { openTab } from '$lib/stores/tabs.svelte';
    import { getEditorMode } from '$lib/stores/ui.svelte';

    let debounceTimer: ReturnType<typeof setTimeout> | null = null;

    function onInput(value: string) {
        setQuery(value);
        if (debounceTimer) clearTimeout(debounceTimer);
        if (!value.trim()) {
            setResults([]);
            return;
        }
        debounceTimer = setTimeout(async () => {
            try {
                setSearching(true);
                const r = await searchNotes(value);
                setResults(r);
            } catch (e) {
                console.error('Search failed:', e);
            } finally {
                setSearching(false);
            }
        }, 300);
    }

    async function handleClick(result: { path: string; title: string }) {
        try {
            const content = await readNote(result.path);
            setActiveNote(result.path);
            setContent(content);
            markClean();
            openTab({
                path: result.path,
                title: result.title,
                isDirty: false,
                mode: getEditorMode()
            });
        } catch (e) {
            console.error('Failed to open search result:', e);
        }
    }
</script>

<div class="search-panel">
    <input
        type="text"
        class="search-input"
        placeholder="Search notes..."
        value={getQuery()}
        oninput={(e) => onInput(e.currentTarget.value)}
    />

    {#if isSearching()}
        <p class="status">Searching...</p>
    {/if}

    {#if getResults().length > 0}
        <div class="results">
            {#each getResults() as result}
                <button class="result-item" onclick={() => handleClick(result)}>
                    <span class="result-title">{result.title}</span>
                    <span class="result-snippet">{@html result.snippet}</span>
                </button>
            {/each}
        </div>
    {:else if getQuery() && !isSearching()}
        <p class="status">No results</p>
    {/if}
</div>

<style>
    .search-panel {
        padding: 8px;
    }
    .search-input {
        width: 100%;
        padding: 6px 10px;
        font-size: 13px;
        border: 1px solid var(--border-color, #444);
        background: var(--bg-primary, #1e1e1e);
        color: inherit;
        border-radius: 4px;
        box-sizing: border-box;
    }
    .search-input:focus {
        outline: none;
        border-color: var(--accent, #61afef);
    }
    .status {
        padding: 8px;
        font-size: 12px;
        color: var(--text-muted, #888);
    }
    .results {
        margin-top: 8px;
    }
    .result-item {
        display: block;
        width: 100%;
        background: none;
        border: none;
        text-align: left;
        padding: 6px 8px;
        cursor: pointer;
        border-radius: 4px;
        font-size: 12px;
    }
    .result-item:hover {
        background: var(--hover-bg, #444);
    }
    .result-title {
        display: block;
        font-weight: 600;
        color: var(--text-primary, #d4d4d4);
    }
    .result-snippet {
        display: block;
        color: var(--text-muted, #888);
        font-size: 11px;
        margin-top: 2px;
    }
</style>
