import { marked, Renderer } from 'marked';

interface WikilinkToken {
    type: 'wikilink';
    raw: string;
    text: string;
    target: string;
}

function wikilinkExtension() {
    return {
        name: 'wikilink',
        level: 'inline' as const,
        start(src: string) {
            return src.indexOf('[[');
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

marked.use({ extensions: [wikilinkExtension()] });

export function renderMarkdown(content: string): string {
    return marked.parse(content, { async: false }) as string;
}
