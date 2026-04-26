import type { EditorMode } from '$lib/types';

/// A single open editor tab, identified by its file path.
export interface Tab {
    path: string;
    title: string;
    isDirty: boolean;
    mode: EditorMode;
}

/// Per-tab scroll position, preserved independently for edit and reading modes.
export interface ScrollState {
    edit: number;
    reading: number | null;
}

// Non-reactive per-tab scroll state (not $state, avoids reactivity overhead on scroll events)
const scrollStates = new Map<string, ScrollState>();

const state = $state({
    tabs: [] as Tab[],
    activeTabIndex: 0
});

const activeTab = $derived(state.tabs[state.activeTabIndex] ?? null);

export function getTabs() { return state.tabs; }
export function getActiveTabIndex() { return state.activeTabIndex; }
export function getActiveTab() { return activeTab; }
export function getTabByPath(path: string): Tab | undefined {
    return state.tabs.find(t => t.path === path);
}
export function findTabIndex(path: string): number {
    return state.tabs.findIndex(t => t.path === path);
}

export function openTab(tab: Tab) {
    const existing = state.tabs.findIndex((t) => t.path === tab.path);
    if (existing >= 0) {
        state.activeTabIndex = existing;
    } else {
        state.tabs = [...state.tabs, tab];
        state.activeTabIndex = state.tabs.length - 1;
    }
}

export function openNewTab(tab: Tab) {
    state.tabs = [...state.tabs, tab];
    state.activeTabIndex = state.tabs.length - 1;
}

export function closeTab(index: number) {
    const tab = state.tabs[index];
    state.tabs = state.tabs.filter((_, i) => i !== index);
    if (tab) cleanupScrollState(tab.path);
    if (state.activeTabIndex >= state.tabs.length) {
        state.activeTabIndex = Math.max(0, state.tabs.length - 1);
    }
    if (state.tabs.length === 0) {
        state.activeTabIndex = 0;
    }
}

export function setActive(index: number) {
    state.activeTabIndex = index;
}

export function setDirty(index: number, dirty: boolean) {
    state.tabs = state.tabs.map((t, i) => (i === index ? { ...t, isDirty: dirty } : t));
}

export function setMode(index: number, mode: EditorMode) {
    state.tabs = state.tabs.map((t, i) => (i === index ? { ...t, mode } : t));
}

export function getScrollState(path: string): ScrollState {
    let s = scrollStates.get(path);
    if (!s) {
        s = { edit: 0, reading: null };
        scrollStates.set(path, s);
    }
    return s;
}

export function setScrollEdit(path: string, ratio: number) {
    getScrollState(path).edit = ratio;
}

export function setScrollReading(path: string, ratio: number) {
    getScrollState(path).reading = ratio;
}

// Call from closeTab cleanup
function cleanupScrollState(path: string) {
    scrollStates.delete(path);
}

export { cleanupScrollState as discardScrollState };
