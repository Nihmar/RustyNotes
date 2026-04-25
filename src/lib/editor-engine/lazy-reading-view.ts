import { marked, Lexer, type Tokens } from 'marked';
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

export function splitMarkdownIntoSections(raw: string): MarkdownSection[] {
    if (!raw.trim()) return [];

    const tokens = Lexer.lex(raw, { silent: true });
    const sections: MarkdownSection[] = [];

    let currentSectionTokens: Tokens[] = [];
    let sectionStartIndex = 0;
    let currentHeading = '';
    let currentHeadingLevel: 1 | 2 = 1;
    let firstHeadingSkipped = false;
    let contentHash = raw.slice(0, 32).replace(/[^a-zA-Z0-9]/g, '_');

    function flushSection(tokensToFlush: Tokens[], startIdx: number, heading: string, level: 1 | 2, isFirst: boolean) {
        if (tokensToFlush.length === 0) return;
        const sectionRaw = tokensToFlush
            .map(t => (t as Tokens.Heading).raw ?? '')
            .join('');
        if (!sectionRaw.trim()) return;

        const lines = countLines(sectionRaw);
        const estimatedHeight = Math.ceil(lines * 26 + 60);

        sections.push({
            id: `section-${contentHash}-${sections.length}`,
            heading: isFirst && heading ? heading : heading,
            headingLevel: level,
            rawContent: sectionRaw,
            estimatedHeight
        });
    }

    for (const token of tokens) {
        if (token.type === 'heading') {
            const h = token as Tokens.Heading;
            if (h.depth === 1 || h.depth === 2) {
                if (currentSectionTokens.length > 0) {
                    flushSection(currentSectionTokens, sectionStartIndex, currentHeading, currentHeadingLevel, firstHeadingSkipped);
                    currentSectionTokens = [];
                }
                currentHeading = h.text;
                currentHeadingLevel = h.depth as 1 | 2;
                sectionStartIndex = sections.length;
                if (!firstHeadingSkipped) firstHeadingSkipped = true;
            }
        }
        currentSectionTokens.push(token);
    }

    if (currentSectionTokens.length > 0) {
        flushSection(currentSectionTokens, sectionStartIndex, currentHeading, currentHeadingLevel, firstHeadingSkipped);
    }

    if (sections.length === 0) {
        sections.push({
            id: `section-${contentHash}-0`,
            heading: '',
            headingLevel: 1,
            rawContent: raw,
            estimatedHeight: Math.ceil(countLines(raw) * 26 + 60)
        });
    }

    return sections;
}

export function renderSection(raw: string): string {
    return marked.parse(raw, { async: false }) as string;
}