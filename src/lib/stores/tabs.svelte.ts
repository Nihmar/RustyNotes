import type { EditorMode } from '$lib/types';

export interface Tab {
    path: string;
    title: string;
    isDirty: boolean;
    mode: EditorMode;
}

const state = $state({
    tabs: [] as Tab[],
    activeTabIndex: 0
});

const activeTab = $derived(state.tabs[state.activeTabIndex] ?? null);

export function getTabs() { return state.tabs; }
export function getActiveTabIndex() { return state.activeTabIndex; }
export function getActiveTab() { return activeTab; }

export function openTab(tab: Tab) {
    const existing = state.tabs.findIndex((t) => t.path === tab.path);
    if (existing >= 0) {
        state.activeTabIndex = existing;
    } else {
        state.tabs = [...state.tabs, tab];
        state.activeTabIndex = state.tabs.length - 1;
    }
}

export function closeTab(index: number) {
    state.tabs = state.tabs.filter((_, i) => i !== index);
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
