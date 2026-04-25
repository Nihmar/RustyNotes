<script lang="ts">
    import { onMount } from 'svelte';
    import {
        splitMarkdownIntoSections,
        renderSection,
        type MarkdownSection
    } from '$lib/editor-engine/lazy-reading-view';

    let { content = '' }: { content: string } = $props();

    let sections: MarkdownSection[] = $state([]);
    let rendered: string[] = $state([]);
    let container: HTMLDivElement | undefined = $state();
    let observer: IntersectionObserver | null = null;
    let currentContent = '';

    $effect(() => {
        const newSections = splitMarkdownIntoSections(content);
        if (newSections.length !== sections.length ||
            newSections.some((s, i) => s.rawContent !== sections[i]?.rawContent)) {
            sections = newSections;
            rendered = new Array(newSections.length).fill('');
            currentContent = content;

            if (observer) {
                observer.disconnect();
            }
        }
    });

    $effect(() => {
        const el = container;
        if (!el || sections.length === 0) return;

        if (observer) {
            observer.disconnect();
        }

        observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (!entry.isIntersecting) continue;
                    const idx = parseInt(entry.target.getAttribute('data-section-index') ?? '-1', 10);
                    if (idx < 0 || idx >= sections.length) continue;
                    if (rendered[idx]) {
                        observer?.unobserve(entry.target);
                        continue;
                    }
                    try {
                        rendered[idx] = renderSection(sections[idx].rawContent);
                    } catch (e) {
                        console.error('Failed to parse section', idx, e);
                        rendered[idx] = `<p>Error rendering section.</p>`;
                    }
                    observer?.unobserve(entry.target);
                }
            },
            {
                rootMargin: '300px 0px'
            }
        );

        el.querySelectorAll('[data-section-index]').forEach(child => {
            observer!.observe(child);
        });

        return () => {
            observer?.disconnect();
            observer = null;
        };
    });

    onMount(() => {
        return () => {
            observer?.disconnect();
            observer = null;
        };
    });
</script>

<div class="lazy-reading-view" bind:this={container}>
    {#each sections as section, i (section.id)}
        <div class="section-container" data-section-index={i}>
            {#if rendered[i]}
                {@html rendered[i]}
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