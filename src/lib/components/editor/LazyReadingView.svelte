<script lang="ts">
    import { onMount, onDestroy, tick } from 'svelte';
    import {
        splitMarkdownIntoSections,
        renderSection,
        type MarkdownSection
    } from '$lib/editor-engine/lazy-reading-view';

    let { content = '' }: { content: string } = $props();

    let sections: MarkdownSection[] = $state([]);
    let loadedHtml: Map<string, string> = $state(new Map());
    let container: HTMLDivElement | undefined = $state();
    let observer: IntersectionObserver | null = $state(null);
    let destroyed = false;

    let queue: string[] = [];
    let draining = false;

    function schedule(id: string) {
        if (destroyed || loadedHtml.has(id) || queue.includes(id)) return;
        queue.push(id);
        if (!draining) {
            draining = true;
            requestAnimationFrame(drain);
        }
    }

    function drain() {
        if (destroyed) {
            queue = [];
            draining = false;
            return;
        }

        const budget = 12; // ms
        const deadline = performance.now() + budget;

        while (queue.length > 0 && performance.now() < deadline) {
            const id = queue.shift()!;
            const section = sections.find(s => s.id === id);
            if (!section || loadedHtml.has(id)) continue;
            try {
                loadedHtml.set(id, renderSection(section.rawContent));
            } catch (e) {
                console.error('Failed to parse section', id, e);
            }
        }

        if (queue.length > 0) {
            requestAnimationFrame(drain);
        } else {
            draining = false;
        }
    }

    $effect(() => {
        sections = splitMarkdownIntoSections(content);
        loadedHtml = new Map();
        queue = [];
        draining = false;

        observer?.disconnect();

        if (sections.length > 0) {
            schedule(sections[0].id);
        }
    });

    $effect(() => {
        const obs = observer;
        const el = container;
        if (!obs || !el || sections.length === 0) return;

        tick().then(() => {
            if (destroyed || !obs || !el) return;
            el.querySelectorAll('[data-section-id]').forEach(child => {
                obs.observe(child);
            });
        });
    });

    onMount(() => {
        const root = container?.closest('.editor-wrapper') ?? null;

        observer = new IntersectionObserver(
            (entries) => {
                if (destroyed) return;
                for (const entry of entries) {
                    const id = entry.target.getAttribute('data-section-id');
                    if (!id || !entry.isIntersecting) continue;
                    if (loadedHtml.has(id)) {
                        observer?.unobserve(entry.target);
                        continue;
                    }
                    const section = sections.find(s => s.id === id);
                    if (!section) continue;
                    schedule(id);
                    observer?.unobserve(entry.target);
                }
            },
            {
                root,
                rootMargin: '200% 0px'
            }
        );
    });

    onDestroy(() => {
        destroyed = true;
        observer?.disconnect();
        queue = [];
        draining = false;
    });
</script>

<div class="lazy-reading-view" bind:this={container}>
    {#each sections as section (section.id)}
        <div class="section-container" data-section-id={section.id}>
            {#if loadedHtml.has(section.id)}
                {@html loadedHtml.get(section.id) ?? ''}
            {:else}
                <div
                    class="section-placeholder"
                    style="min-height: {section.estimatedHeight}px"
                >
                    {#if section.heading}
                        <div class="placeholder-heading h{section.headingLevel}">
                            {section.heading}
                        </div>
                    {/if}
                </div>
            {/if}
        </div>
    {/each}
</div>

<style>
    .lazy-reading-view {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 15px;
        line-height: 1.7;
        overflow-wrap: break-word;
    }

    .section-placeholder {
        opacity: 0.6;
    }

    .placeholder-heading {
        font-weight: 600;
        margin: 0.5em 0;
    }

    .placeholder-heading.h1 {
        font-size: 2em;
    }

    .placeholder-heading.h2 {
        font-size: 1.5em;
    }
</style>
