import { FunctionComponent, ReactNode } from 'react';

type CodeBlockProps = {
  content: ReactNode;
  label?: string;
  withCopyBtn?: boolean;
};

const CodeBlock: FunctionComponent<CodeBlockProps> = ({ content, label, withCopyBtn }) => {
  return (
    <div data-testid="code" data-label={label}>
      {content}
      {withCopyBtn && label ? (
        <button aria-label={label} type="button">
          {label}
        </button>
      ) : null}
    </div>
  );
};

export { CodeBlock };
