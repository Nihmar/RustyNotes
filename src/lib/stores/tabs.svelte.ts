import type { EditorMode } from '$lib/types';

export interface Tab {
    path: string;
    title: string;
    isDirty: boolean;
    mode: EditorMode;
}

let tabs = $state<Tab[]>([]);
let activeTabIndex = $state(0);

let activeTab = $derived(tabs[activeTabIndex] ?? null);

export function openTab(tab: Tab) {
    const existing = tabs.findIndex((t) => t.path === tab.path);
    if (existing >= 0) {
        activeTabIndex = existing;
    } else {
        tabs = [...tabs, tab];
        activeTabIndex = tabs.length - 1;
    }
}

export function closeTab(index: number) {
    tabs = tabs.filter((_, i) => i !== index);
    if (activeTabIndex >= tabs.length) {
        activeTabIndex = Math.max(0, tabs.length - 1);
    }
    if (tabs.length === 0) {
        activeTabIndex = 0;
    }
}

export function setActive(index: number) {
    activeTabIndex = index;
}

export function setDirty(index: number, dirty: boolean) {
    tabs = tabs.map((t, i) => (i === index ? { ...t, isDirty: dirty } : t));
}

export function setMode(index: number, mode: EditorMode) {
    tabs = tabs.map((t, i) => (i === index ? { ...t, mode } : t));
}

export { tabs, activeTabIndex, activeTab };
