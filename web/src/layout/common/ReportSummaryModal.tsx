import { useState } from 'react';
import { FiMoon, FiSun } from 'react-icons/fi';

import { Foundation } from '../../types';
import CodeBlock from './CodeBlock';
import Loading from './Loading';
import Modal from './Modal';
import styles from './ReportSummaryModal.module.css';
import Tabs from './Tabs';

interface OpenModalStatus {
  status: boolean;
  name?: string;
}

interface ReportSummaryTheme {
  name: string;
  icon: JSX.Element;
}

const DEFAULT_THEME = 'light';
const THEMES: ReportSummaryTheme[] = [
  {
    name: DEFAULT_THEME,
    icon: <FiSun />,
  },
  { name: 'dark', icon: <FiMoon /> },
];

interface Props {
  foundation: Foundation;
  projectName: string;
  openStatus: OpenModalStatus;
  onCloseModal: () => void;
}

const ReportSummaryModal = (props: Props) => {
  const origin = window.location.origin;
  const [theme, setTheme] = useState<string>(DEFAULT_THEME);
  const image = `${origin}/api/projects/${props.foundation}/${props.projectName}/report-summary?theme=${theme}`;
  const projectLink = `${origin}/projects/${props.foundation}/${props.projectName}`;
  const markdownLink = `[![CLOMonitor report summary](${image})](${projectLink})`;
  const asciiLink = `${projectLink}[image:${image}[CLOMonitor report summary]]`;
  const htmlLink = `<a href="${projectLink}" rel="noopener noreferrer" target="_blank"><img src="${image}" alt="CLOMonitor report summary" /></a>`;
  const [imageLoaded, setImageLoaded] = useState<boolean>(false);

  const onCloseModal = () => {
    props.onCloseModal();
    setTheme(DEFAULT_THEME);
    setImageLoaded(false);
  };

  return (
    <Modal
      header="Embed report summary"
      onClose={onCloseModal}
      open={props.openStatus.status && props.openStatus.name === 'reportSummary'}
    >
      <div className="w-100 position-relative">
        <label
          className={`w-100 text-primary text-uppercase fw-bold border-bottom mb-3 ${styles.label}`}
          htmlFor="theme"
        >
          Theme
        </label>
        <div className="d-flex flex-row mb-3">
          {THEMES.map((themeOpt: ReportSummaryTheme) => {
            return (
              <div className="form-check me-4" key={`radio_theme_${themeOpt.name}`}>
                <input
                  className="form-check-input"
                  type="radio"
                  name="theme"
                  id={themeOpt.name}
                  value={themeOpt.name}
                  checked={theme === themeOpt.name}
                  onChange={() => {
                    setImageLoaded(false);
                    setTheme(themeOpt.name);
                  }}
                  required
                  readOnly
                />
                <label className="form-label text-capitalize form-check-label" htmlFor={themeOpt.name}>
                  <div className="d-flex flex-row align-items-center">
                    {themeOpt.icon}
                    <span className="ms-1">{themeOpt.name}</span>
                  </div>
                </label>
              </div>
            );
          })}
        </div>

        <div className="mt-4">
          <label className={`w-100 text-primary text-uppercase fw-bold border-bottom mb-3 ${styles.label}`}>Code</label>
          <Tabs
            className="pt-2"
            tabs={[
              {
                name: 'markdown',
                title: 'Markdown',
                content: (
                  <CodeBlock
                    language="markdown"
                    content={markdownLink}
                    label="Copy report summary markdown link to clipboard"
                    withCopyBtn
                  />
                ),
              },

              {
                name: 'ascii',
                title: 'AsciiDoc',
                content: (
                  <CodeBlock
                    language="asciidoc"
                    content={asciiLink}
                    label="Copy report summary Ascii link to clipboard"
                    withCopyBtn
                  />
                ),
              },
              {
                name: 'html',
                title: 'HTML',
                content: (
                  <CodeBlock
                    language="html"
                    content={htmlLink}
                    label="Copy report summary html link to clipboard"
                    withCopyBtn
                  />
                ),
              },
            ]}
            active="markdown"
            noDataContent="Sorry, the information for this is missing."
          />
        </div>

        <div className="mt-4 d-flex flex-column">
          <label className={`text-primary text-uppercase fw-bold border-bottom mb-4 ${styles.label}`}>Preview</label>

          <div className={`text-center mx-auto my-3 position-relative w-100 ${styles.imgWrapper}`}>
            {!imageLoaded && <Loading />}
            <img src={image} alt="CLOMonitor report summary" className="border" onLoad={() => setImageLoaded(true)} />
          </div>
        </div>
      </div>
    </Modal>
  );
};

export default ReportSummaryModal;
