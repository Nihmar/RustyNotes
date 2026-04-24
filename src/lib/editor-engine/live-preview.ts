import {
    Decoration,
    EditorView,
    ViewPlugin,
    ViewUpdate,
    WidgetType
} from '@codemirror/view';
import type { DecorationSet } from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';
import katex from 'katex';

class HrWidget extends WidgetType {
    toDOM() {
        const el = document.createElement('hr');
        el.className = 'cm-lp-hr';
        return el;
    }
    eq() { return true; }
}

class SpanWidget extends WidgetType {
    constructor(private span: HTMLSpanElement) { super(); }
    toDOM() { return this.span; }
    eq(other: SpanWidget) {
        return this.span.outerHTML === other.span.outerHTML;
    }
}

class MathWidget extends WidgetType {
    constructor(private latex: string, private displayMode: boolean) { super(); }
    toDOM() {
        const container = document.createElement(this.displayMode ? 'div' : 'span');
        container.className = this.displayMode ? 'cm-lp-math-block' : 'cm-lp-math-inline';
        try {
            katex.render(this.latex, container, { displayMode: this.displayMode, throwOnError: false });
        } catch {
            container.textContent = `$${this.latex}$`;
        }
        return container;
    }
    eq(other: MathWidget) {
        return this.latex === other.latex && this.displayMode === other.displayMode;
    }
    updateDOM(dom: HTMLElement) {
        try {
            dom.innerHTML = '';
            katex.render(this.latex, dom, { displayMode: this.displayMode, throwOnError: false });
        } catch {
            dom.textContent = `$${this.latex}$`;
        }
        return true;
    }
}

const boldMark = Decoration.mark({ class: 'cm-lp-bold' });
const italicMark = Decoration.mark({ class: 'cm-lp-italic' });
const strikethroughMark = Decoration.mark({ class: 'cm-lp-strikethrough' });
const codeMark = Decoration.mark({ class: 'cm-lp-code' });
const blockquoteMark = Decoration.line({ class: 'cm-lp-blockquote' });
const hrWidget = Decoration.widget({ widget: new HrWidget() });
const hiddenSyntax = Decoration.replace({});

function renderLatex(latex: string, displayMode: boolean): HTMLElement {
    const el = document.createElement(displayMode ? 'div' : 'span');
    el.className = displayMode ? 'cm-lp-math-block' : 'cm-lp-math-inline';
    try {
        katex.render(latex, el, { displayMode, throwOnError: false });
    } catch {
        el.textContent = `$${latex}$`;
    }
    return el;
}

function decorateLivePreview(view: EditorView): DecorationSet {
    const builder = new RangeSetBuilder<Decoration>();
    const doc = view.state.doc;
    const cursorLine = view.state.selection.main.head;
    const cursorLineNum = doc.lineAt(cursorLine).number;

    console.log('[live-preview] decorateLivePreview called, lines:', doc.lines, 'cursor at:', cursorLineNum);

    // First pass: find display math blocks ($$...$$)
    const mathBlockLines = new Set<number>();
    const displayMathBlocks: { startLine: number; endLine: number; latex: string; from: number; to: number }[] = [];
    let inMathBlock = false;
    let mathBlockStart = -1;
    let mathBlockStartFrom = -1;
    const mathLines: string[] = [];

    for (let i = 1; i <= doc.lines; i++) {
        const line = doc.line(i);
        const lineText = line.text;

        if (!inMathBlock && /^\$\$\s*$/.test(lineText)) {
            inMathBlock = true;
            mathBlockStart = i;
            mathBlockStartFrom = line.from;
            mathLines.length = 0;
            mathBlockLines.add(i);
            continue;
        }
        if (inMathBlock && /^\$\$\s*$/.test(lineText)) {
            displayMathBlocks.push({
                startLine: mathBlockStart,
                endLine: i,
                latex: mathLines.join('\n').trim(),
                from: mathBlockStartFrom,
                to: line.to
            });
            mathBlockLines.add(i);
            inMathBlock = false;
            continue;
        }
        if (inMathBlock) {
            mathLines.push(lineText);
        }
    }

    // Process line by line
    for (let i = 1; i <= doc.lines; i++) {
        const line = doc.line(i);
        const lineText = line.text;
        const lineFrom = line.from;
        const isCursorLine = i === cursorLineNum;

        // Skip cursor line
        if (isCursorLine) continue;

        // Skip display math fence lines
        if (mathBlockLines.has(i)) continue;

        // Headings
        const headingMatch = lineText.match(/^(#{1,6})\s+(.*)/);
        if (headingMatch) {
            const level = headingMatch[1].length;
            builder.add(lineFrom, lineFrom + headingMatch[1].length, hiddenSyntax);
            builder.add(
                lineFrom,
                lineFrom + line.length,
                Decoration.line({ class: `cm-lp-h${level}` })
            );
            continue;
        }

        // Horizontal rules
        if (/^[-*_]{3,}\s*$/.test(lineText)) {
            builder.add(lineFrom, line.to, hrWidget);
            continue;
        }

        // Blockquotes
        const bqMatch = lineText.match(/^>\s?(.*)/);
        if (bqMatch) {
            builder.add(lineFrom, lineFrom + bqMatch[0].length - bqMatch[1].length, hiddenSyntax);
            builder.add(lineFrom, line.to, blockquoteMark);
        }

        // Checkbox / task list (must check before unordered list)
        const checkMatch = lineText.match(/^(\s*)[-*+]\s+\[([ xX])\]\s+(.*)/);
        if (checkMatch && !bqMatch) {
            const indent = checkMatch[1].length;
            const checked = checkMatch[2].toLowerCase() === 'x';
            const contentText = checkMatch[3];
            const prefixLen = checkMatch[0].length - contentText.length;
            const contentStart = lineFrom + prefixLen;

            builder.add(lineFrom + indent, contentStart, hiddenSyntax);

            const span = document.createElement('span');
            span.className = 'cm-lp-checkbox';
            span.textContent = checked ? '☑' : '☐';
            builder.add(
                lineFrom + indent,
                lineFrom + indent,
                Decoration.widget({ widget: new SpanWidget(span), side: 0 })
            );
            continue;
        }

        // Unordered list items
        const ulMatch = lineText.match(/^(\s*)([-*+])\s+(.+)/);
        if (ulMatch && !bqMatch) {
            const indent = ulMatch[1].length;
            const contentText = ulMatch[3];
            const prefixLen = ulMatch[0].length - contentText.length;
            const contentStart = lineFrom + prefixLen;

            builder.add(lineFrom + indent, contentStart, hiddenSyntax);

            const bulletSpan = document.createElement('span');
            bulletSpan.className = 'cm-lp-bullet';
            bulletSpan.textContent = '•';
            builder.add(
                lineFrom + indent,
                lineFrom + indent,
                Decoration.widget({ widget: new SpanWidget(bulletSpan), side: 0 })
            );
            continue;
        }

        // Ordered list items
        const olMatch = lineText.match(/^(\s*)(\d+)\.\s+(.+)/);
        if (olMatch && !bqMatch) {
            const indent = olMatch[1].length;
            const contentText = olMatch[3];
            const prefixLen = olMatch[0].length - contentText.length;
            const contentStart = lineFrom + prefixLen;

            builder.add(lineFrom + indent, contentStart, hiddenSyntax);
            continue;
        }

        // Code blocks (triple backtick)
        if (lineText.startsWith('```')) {
            builder.add(lineFrom, line.to, hiddenSyntax);
            continue;
        }
    }

    // Add display math block widgets
    for (const block of displayMathBlocks) {
        const el = renderLatex(block.latex, true);
        builder.add(
            block.from,
            block.to,
            Decoration.widget({ widget: new SpanWidget(el), block: true })
        );
    }

    // Process inline styles: bold, italic, code, strikethrough
    const inlineRegexes = [
        { regex: /\*\*(.+?)\*\*/g, deco: boldMark, group: 1, openLen: 2, closeLen: 2 },
        { regex: /(?<!\*)\*(?!\*)(.+?)(?<!\*)\*(?!\*)/g, deco: italicMark, group: 1, openLen: 1, closeLen: 1 },
        { regex: /__(.+?)__/g, deco: boldMark, group: 1, openLen: 2, closeLen: 2 },
        { regex: /(?<!_)_(?!_)(.+?)(?<!_)_(?!_)/g, deco: italicMark, group: 1, openLen: 1, closeLen: 1 },
        { regex: /`([^`]+)`/g, deco: codeMark, group: 1, openLen: 1, closeLen: 1 },
        { regex: /~~(.+?)~~/g, deco: strikethroughMark, group: 1, openLen: 2, closeLen: 2 },
    ];

    for (let i = 1; i <= doc.lines; i++) {
        if (i === cursorLineNum) continue;
        if (mathBlockLines.has(i)) continue;
        const line = doc.line(i);
        const lineText = line.text;
        const lineFrom = line.from;

        // Inline math $...$ — hide delimiters, render as widget (exclude $$ blocks)
        const inlineMathRegex = /(?<!\$)\$(?!\$)([^$\n]+?)\$(?!\$)/g;
        let mathMatch;
        while ((mathMatch = inlineMathRegex.exec(lineText)) !== null) {
            const absStart = lineFrom + mathMatch.index;
            const absEnd = absStart + mathMatch[0].length;
            const latex = mathMatch[1].trim();
            // Hide both $ delimiters
            builder.add(absStart, absStart + 1, hiddenSyntax);
            builder.add(absEnd - 1, absEnd, hiddenSyntax);
            // Render math widget
            const mathEl = renderLatex(latex, false);
            builder.add(
                absStart + 1,
                absEnd - 1,
                Decoration.widget({ widget: new SpanWidget(mathEl), side: 0 })
            );
        }

        for (const { regex, deco, openLen, closeLen } of inlineRegexes) {
            let match;
            const r = new RegExp(regex.source, regex.flags);
            r.lastIndex = 0;
            while ((match = r.exec(lineText)) !== null) {
                const absStart = lineFrom + match.index;
                const absEnd = absStart + match[0].length;
                // Check overlap with inline math
                const mathInlineStart = lineText.indexOf('$', match.index);
                if (mathInlineStart !== -1 && mathInlineStart < match.index + match[0].length) {
                    continue;
                }
                builder.add(absStart, absStart + openLen, hiddenSyntax);
                builder.add(absEnd - closeLen, absEnd, hiddenSyntax);
                builder.add(absStart + openLen, absEnd - closeLen, deco);
            }
        }
    }

    return builder.finish();
}

const livePreviewPlugin = ViewPlugin.fromClass(class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
        this.decorations = decorateLivePreview(view);
    }

    update(update: ViewUpdate) {
        if (update.docChanged || update.selectionSet || update.viewportChanged) {
            this.decorations = decorateLivePreview(update.view);
        }
    }
}, {
    decorations: (v) => v.decorations
});

export function livePreview() {
    return livePreviewPlugin;
}
