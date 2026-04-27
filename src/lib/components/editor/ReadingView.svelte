<script lang="ts">
    import { onMount } from 'svelte';
    import { renderMarkdown } from '$lib/commands';
    import { navigateWikilink } from '$lib/editor-engine/wikilinks';
    import katex from 'katex';

    let { content = '' }: { content: string } = $props();

    let htmlSections: string[] = $state([]);
    let container: HTMLDivElement | undefined = $state();
    let currentContent = '';
    let renderGeneration = 0;

    function splitHtmlByHeadings(html: string): string[] {
        if (!html) return [];
        const parts: string[] = [];
        let lastEnd = 0;
        const re = /<h[12][ >]/g;
        let m: RegExpExecArray | null;
        while ((m = re.exec(html)) !== null) {
            if (m.index > lastEnd) {
                parts.push(html.slice(lastEnd, m.index));
            }
            lastEnd = m.index;
        }
        if (lastEnd < html.length) {
            parts.push(html.slice(lastEnd));
        }
        return parts.length > 0 ? parts : [html];
    }

    async function startProgressiveInsert(html: string) {
        const sections = splitHtmlByHeadings(html);
        const batchSize = 10;
        const gen = renderGeneration;
        htmlSections = [];

        for (let i = 0; i < sections.length; i += batchSize) {
            if (renderGeneration !== gen) return;
            const batch = sections.slice(i, i + batchSize).join('');
            htmlSections = [...htmlSections, batch];
            if (i + batchSize < sections.length) {
                await new Promise((r) => requestAnimationFrame(r));
            }
        }

        if (renderGeneration === gen) {
            postProcessMath();
        }
    }

    function postProcessMath() {
        if (!container) return;
        container.querySelectorAll<HTMLElement>('.math-block, .math-inline').forEach((mathEl) => {
            if (mathEl.dataset.katexRendered) return;
            const isBlock = mathEl.classList.contains('math-block');
            try {
                katex.render(mathEl.textContent || '', mathEl, {
                    displayMode: isBlock,
                    throwOnError: false
                });
                mathEl.dataset.katexRendered = '1';
            } catch (e) {
                mathEl.classList.add('math-error');
            }
        });
    }

    $effect(() => {
        if (!content || content === currentContent) return;
        currentContent = content;
        const gen = ++renderGeneration;

        renderMarkdown(content)
            .then((result) => {
                if (renderGeneration !== gen || !result) return;
                startProgressiveInsert(result);
            })
            .catch((e) => {
                if (renderGeneration !== gen) return;
                console.error('Render failed:', e);
                htmlSections = ['<p>Error rendering markdown.</p>'];
            });
    });

    onMount(() => {
        function onWikilinkClick(e: MouseEvent) {
            const link = (e.target as HTMLElement).closest('a.wikilink');
            if (!link) return;
            e.preventDefault();
            e.stopPropagation();
            const href = link.getAttribute('href') ?? '';
            const target = decodeURIComponent(href.replace('note://', ''));
            navigateWikilink(target, e.ctrlKey || e.metaKey);
        }

        container?.addEventListener('click', onWikilinkClick);

        return () => {
            container?.removeEventListener('click', onWikilinkClick);
        };
    });
</script>

<div class="reading-view" bind:this={container}>
    {#if htmlSections.length > 0}
        {#each htmlSections as sectionHtml}
            {@html sectionHtml}
        {/each}
    {:else if content}
        <div class="reading-loading">Rendering...</div>
    {/if}
</div>

<style>
    .reading-view {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 15px;
        line-height: 1.7;
        overflow-wrap: break-word;
    }
    .reading-loading {
        opacity: 0.5;
        padding: 20px 0;
        font-style: italic;
    }
</style>
