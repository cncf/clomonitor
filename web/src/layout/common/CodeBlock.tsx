import { isUndefined } from 'lodash';
import { useContext } from 'react';
import SyntaxHighlighter from 'react-syntax-highlighter';
import { docco } from 'react-syntax-highlighter/dist/cjs/styles/hljs';
import { tomorrowNight } from 'react-syntax-highlighter/dist/cjs/styles/hljs';

import { AppContext } from '../../context/AppContextProvider';
import ButtonCopyToClipboard from './ButtonCopyToClipboard';
import styles from './CodeBlock.module.css';

interface Props {
  language: string;
  content: string;
  withCopyBtn: boolean;
  label?: string;
  darkCode?: boolean;
}

const CodeBlock = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const isDarkActive = (effective === 'dark' && isUndefined(props.darkCode)) || props.darkCode;

  return (
    <div data-testid="code" className={`d-flex flex-row align-items-center pb-2 ${styles.codeBlock}`}>
      <SyntaxHighlighter
        language={props.language}
        style={isDarkActive ? tomorrowNight : docco}
        customStyle={{
          backgroundColor: 'var(--bg-code)',
          color: 'var(--color-font)',
          padding: '1rem 0.5rem',
          marginBottom: 0,
          width: props.withCopyBtn ? 'calc(100% - 1rem - 32px)' : '100%',
        }}
      >
        {props.content}
      </SyntaxHighlighter>

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
