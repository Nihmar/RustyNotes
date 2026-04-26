import { marked } from 'marked';
import '$lib/editor-engine/reading-view';

export interface MarkdownSection {
    id: string;
    heading: string;
    headingLevel: 1 | 2;
    rawContent: string;
    estimatedHeight: number;
}

function countLines(text: string): number {
    let count = 0;
    for (let i = 0; i < text.length; i++) {
        if (text[i] === '\n') count++;
    }
    return Math.max(count, 1);
}

const headingRegex = /^(#{1,2})\s+(.+)$/;

export function splitMarkdownIntoSections(raw: string): MarkdownSection[] {
    if (!raw.trim()) return [];

    const contentHash = raw.slice(0, 32).replace(/[^a-zA-Z0-9]/g, '_');
    const sections: MarkdownSection[] = [];

    let currentHeading = '';
    let currentHeadingLevel: 1 | 2 = 1;
    let currentLines: string[] = [];

    function flushSection() {
        if (currentLines.length === 0) return;
        const sectionRaw = currentLines.join('\n');
        if (!sectionRaw.trim()) return;

        const lines = countLines(sectionRaw);
        const estimatedHeight = Math.ceil(lines * 26 + 60);

        sections.push({
            id: `section-${contentHash}-${sections.length}`,
            heading: currentHeading,
            headingLevel: currentHeadingLevel,
            rawContent: sectionRaw,
            estimatedHeight,
        });
    }

    for (const line of raw.split(/\r?\n/)) {
        const match = headingRegex.exec(line);
        if (match) {
            flushSection();
            currentHeadingLevel = match[1].length as 1 | 2;
            currentHeading = match[2];
            currentLines = [line];
        } else {
            currentLines.push(line);
        }
    }

    flushSection();

    if (sections.length === 0) {
        sections.push({
            id: `section-${contentHash}-0`,
            heading: '',
            headingLevel: 1,
            rawContent: raw,
            estimatedHeight: Math.ceil(countLines(raw) * 26 + 60),
        });
    }

    return sections;
}

export function renderSection(raw: string): string {
    return marked.parse(raw, { async: false }) as string;
}
