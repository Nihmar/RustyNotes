import { marked } from 'marked';
import katex from 'katex';

interface WikilinkToken {
    type: 'wikilink';
    raw: string;
    text: string;
    target: string;
}

interface LatexBlockToken {
    type: 'latexBlock';
    raw: string;
    text: string;
}

interface LatexInlineToken {
    type: 'latexInline';
    raw: string;
    text: string;
}

interface ImageEmbedToken {
    type: 'imageEmbed';
    raw: string;
    target: string;
}

function wikilinkExtension() {
    return {
        name: 'wikilink',
        level: 'inline' as const,
        start(src: string) {
            const idx = src.indexOf('[[');
            if (idx === -1) return -1;
            if (idx > 0 && src[idx - 1] === '!') {
                const next = src.indexOf('[[', idx + 2);
                return next >= 0 ? next : -1;
            }
            return idx;
        },
        tokenizer(src: string) {
            const match = src.match(/^\[\[([^\]]+)\]\]/);
            if (match) {
                const raw = match[0];
                const target = match[1];
                const text = target.includes('|') ? target.split('|')[1] : target;
                return {
                    type: 'wikilink',
                    raw,
                    text,
                    target: target.includes('|') ? target.split('|')[0] : target
                };
            }
        },
        renderer(token: WikilinkToken) {
            return `<a class="wikilink" href="note://${encodeURIComponent(token.target)}">${token.text}</a>`;
        }
    };
}

function latexBlockExtension() {
    return {
        name: 'latexBlock',
        level: 'block' as const,
        start(src: string) {
            return src.indexOf('$$');
        },
        tokenizer(src: string) {
            const match = src.match(/^\$\$\n?([\s\S]*?)\n?\$\$/);
            if (match) {
                return {
                    type: 'latexBlock',
                    raw: match[0],
                    text: match[1].trim()
                };
            }
        },
        renderer(token: LatexBlockToken) {
            try {
                return `<div class="math-block">${katex.renderToString(token.text, { displayMode: true, throwOnError: false })}</div>`;
            } catch {
                return `<div class="math-block math-error">${token.raw}</div>`;
            }
        }
    };
}

function latexInlineExtension() {
    return {
        name: 'latexInline',
        level: 'inline' as const,
        start(src: string) {
            return src.indexOf('$');
        },
        tokenizer(src: string) {
            const match = src.match(/^\$(?!\$)([^$\n]+?)\$(?!\$)/);
            if (match) {
                return {
                    type: 'latexInline',
                    raw: match[0],
                    text: match[1].trim()
                };
            }
        },
        renderer(token: LatexInlineToken) {
            try {
                return `<span class="math-inline">${katex.renderToString(token.text, { displayMode: false, throwOnError: false })}</span>`;
            } catch {
                return `<span class="math-inline math-error">${token.raw}</span>`;
            }
        }
    };
}

function imageEmbedExtension() {
    return {
        name: 'imageEmbed',
        level: 'inline' as const,
        start(src: string) {
            return src.indexOf('![[');
        },
        tokenizer(src: string) {
            const match = src.match(/^!\[\[([^\]]+)\]\]/);
            if (match) {
                const inner = match[1].trim();
                const target = inner.includes('|') ? inner.split('|')[0].trim() : inner;
                return {
                    type: 'imageEmbed',
                    raw: match[0],
                    target
                };
            }
        },
        renderer(token: ImageEmbedToken) {
            return `<img class="image-embed" src="vault://localhost/${encodeURIComponent(token.target)}" alt="${token.target}" loading="lazy">`;
        }
    };
}

const EXTENSIONS_REGISTERED = Symbol.for('rustynotes.marked.extensions');

if (!(globalThis as Record<string, unknown>)[EXTENSIONS_REGISTERED as unknown as string]) {
    marked.use({ extensions: [imageEmbedExtension(), wikilinkExtension(), latexBlockExtension(), latexInlineExtension()] });
    (globalThis as Record<string, unknown>)[EXTENSIONS_REGISTERED as unknown as string] = true;
}

export function renderMarkdown(content: string): string {
    return marked.parse(content, { async: false }) as string;
}
