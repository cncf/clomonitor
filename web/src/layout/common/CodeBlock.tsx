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
  label: string;
  styles?: { [key: string]: string };
}

const CodeBlock = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const isDarkActive = effective === 'dark';

  return (
    <div className={`d-flex flex-row align-items-center pb-2 ${styles.codeBlock}`}>
      <SyntaxHighlighter
        language={props.language}
        style={isDarkActive ? tomorrowNight : docco}
        customStyle={{
          backgroundColor: 'var(--bg-code)',
          color: 'var(--color-font)',
          padding: '1rem 0.5rem',
          marginBottom: 0,
          ...props.styles,
        }}
      >
        {props.content}
      </SyntaxHighlighter>

      <ButtonCopyToClipboard text={props.content} label={props.label} wrapperClassName="ms-3" />
    </div>
  );
};

export default CodeBlock;
