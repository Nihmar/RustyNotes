import { marked } from 'marked';
import '$lib/editor-engine/reading-view';

export interface MarkdownSection {
    id: string;
    heading: string;
    headingLevel: 1 | 2;
    rawContent: string;
    estimatedHeight: number;
}

/**
 * Split raw markdown into sections at h1/h2 heading boundaries.
 * Uses a regex split that keeps each heading with its following content.
 *
 * Known limitation: fenced code blocks containing lines that look like
 * headings (`# ` or `## ` at line start) will cause false splits.
 * A Lezer-based AST walk would handle this correctly but would be
 * significantly more complex. In practice this edge case is rare.
 */
export function splitMarkdownIntoSections(raw: string): MarkdownSection[] {
    const parts = raw.split(/^(?=#{1,2}\s)/m);

    return parts
        .filter(part => part.trim().length > 0)
        .map((part, i) => {
            const headingMatch = part.match(/^(#{1,2})\s+(.+)$/m);
            const level = headingMatch
                ? (headingMatch[1].length as 1 | 2)
                : 1;
            const heading = headingMatch ? headingMatch[2].trim() : '';

            const lines = Math.max(part.length / 65, 1);
            const estimatedHeight = Math.ceil(lines * 26 + 60);

            return {
                id: `section-${i}`,
                heading,
                headingLevel: level,
                rawContent: part,
                estimatedHeight
            };
        });
}

/**
 * Parse a single section's raw markdown to HTML.
 * Uses the same `marked` instance (with registered extensions from reading-view.ts)
 * so wiki-links, LaTeX, etc. render correctly within each section.
 */
export function renderSection(raw: string): string {
    return marked.parse(raw, { async: false }) as string;
}
