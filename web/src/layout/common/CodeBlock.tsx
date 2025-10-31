import { ButtonCopyToClipboard } from 'clo-ui/components/ButtonCopyToClipboard';
import isUndefined from 'lodash/isUndefined';
import { useEffect, useMemo, useState } from 'react';

import styles from './CodeBlock.module.css';

type SyntaxHighlighterModule = (typeof import('react-syntax-highlighter/dist/esm/light'))['default'];
type HighlighterStyle = (typeof import('react-syntax-highlighter/dist/esm/styles/hljs/docco'))['default'];

interface HighlighterBundle {
  component: SyntaxHighlighterModule;
  styles: { light: HighlighterStyle; dark: HighlighterStyle };
}

export interface CodeBlockProps {
  language: string;
  content: string;
  withCopyBtn: boolean;
  effective_theme: string;
  label?: string;
  darkCode?: boolean;
  style?: { [key: string]: string | number };
}

const defaultCustomStyle = {
  backgroundColor: 'var(--bg-code)',
  color: 'var(--color-font)',
  padding: '1rem',
  marginBottom: 0,
  flex: '1 1 auto',
  whiteSpace: 'pre-wrap',
  wordBreak: 'break-word',
  overscrollBehavior: 'contain',
} as const;

let highlighterBundlePromise: Promise<HighlighterBundle> | null = null;

const loadHighlighterBundle = () => {
  if (!highlighterBundlePromise) {
    highlighterBundlePromise = Promise.all([
      import('react-syntax-highlighter/dist/esm/light'),
      import('react-syntax-highlighter/dist/esm/languages/hljs/markdown'),
      import('react-syntax-highlighter/dist/esm/languages/hljs/asciidoc'),
      import('react-syntax-highlighter/dist/esm/languages/hljs/xml'),
      import('react-syntax-highlighter/dist/esm/styles/hljs/docco'),
      import('react-syntax-highlighter/dist/esm/styles/hljs/tomorrow-night'),
    ]).then(([lightModule, markdownModule, asciidocModule, htmlModule, doccoModule, tomorrowNightModule]) => {
      const SyntaxHighlighter = lightModule.default;
      SyntaxHighlighter.registerLanguage('markdown', markdownModule.default);
      SyntaxHighlighter.registerLanguage('asciidoc', asciidocModule.default);
      SyntaxHighlighter.registerLanguage('html', htmlModule.default);

      return {
        component: SyntaxHighlighter,
        styles: { light: doccoModule.default, dark: tomorrowNightModule.default },
      };
    });
  }

  return highlighterBundlePromise;
};

export const CodeBlock = (props: CodeBlockProps) => {
  const [bundle, setBundle] = useState<HighlighterBundle | null>(null);
  const shouldHighlight = Boolean(props.language);

  useEffect(() => {
    if (!shouldHighlight) {
      setBundle(null);
      return;
    }

    let isMounted = true;

    loadHighlighterBundle().then((loadedBundle) => {
      if (isMounted) {
        setBundle(loadedBundle);
      }
    });

    return () => {
      isMounted = false;
    };
  }, [shouldHighlight]);

  const customStyle = useMemo(
    () => ({
      ...defaultCustomStyle,
      width: props.withCopyBtn ? 'calc(100% - 1rem - 32px)' : '100%',
      ...props.style,
    }),
    [props.style, props.withCopyBtn]
  );

  const isDarkActive = (props.effective_theme === 'dark' && isUndefined(props.darkCode)) || props.darkCode;
  const SyntaxHighlighterComponent = bundle?.component;
  const selectedStyle = bundle ? (isDarkActive ? bundle.styles.dark : bundle.styles.light) : null;

  return (
    <div data-testid="code" className={`d-flex flex-row align-items-center pb-2 ${styles.codeBlock}`}>
      {shouldHighlight && SyntaxHighlighterComponent && selectedStyle ? (
        <SyntaxHighlighterComponent
          language={props.language}
          style={selectedStyle}
          customStyle={customStyle}
          className="mb-0 flex-grow-1"
        >
          {props.content}
        </SyntaxHighlighterComponent>
      ) : (
        <pre className="mb-0 flex-grow-1" style={customStyle}>
          {props.content}
        </pre>
      )}

      {props.withCopyBtn && (
        <ButtonCopyToClipboard
          text={props.content}
          label={props.label || 'Copy code to clipboard'}
          wrapperClassName="ms-3"
        />
      )}
    </div>
  );
};

export default CodeBlock;
