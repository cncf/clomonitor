import SyntaxHighlighter from 'react-syntax-highlighter';
import { docco } from 'react-syntax-highlighter/dist/cjs/styles/hljs';

import ButtonCopyToClipboard from './ButtonCopyToClipboard';
import Modal from './Modal';
import Tabs from './Tabs';

interface OpenModalStatus {
  status: boolean;
  name?: string;
}

enum Modals {
  Badge = 'badge',
  Embed = 'embed',
}

interface Props {
  orgName: string;
  projectName: string;
  openStatus: OpenModalStatus;
  onCloseModal: () => void;
}

const BadgeModal = (props: Props) => {
  const origin = window.location.origin;
  const badgeImage = `https://img.shields.io/endpoint?url=${origin}/api/projects/${props.orgName}/${props.projectName}/badge`;
  const markdownLink = `[![CloMonitor](${badgeImage})](${origin}/projects/${props.orgName}/${props.projectName})`;
  const asciiLink = `${origin}/projects/${props.orgName}/${props.projectName}[image:${badgeImage}[CloMonitor]]`;

  return (
    <Modal
      header="Project badge"
      onClose={props.onCloseModal}
      open={props.openStatus.status && props.openStatus.name === Modals.Badge}
    >
      <div className="my-3">
        <Tabs
          tabs={[
            {
              name: 'markdown',
              title: 'Markdown',
              content: (
                <>
                  <div className="mt-2 mb-4">
                    <img src={badgeImage} alt="CloMonitor badge" />
                  </div>

                  <div className="d-flex flex-row align-items-center">
                    <SyntaxHighlighter
                      language="bash"
                      style={docco}
                      customStyle={{
                        backgroundColor: 'var(--color-black-10)',
                        color: 'var(--color-font)',
                        marginBottom: '0',
                      }}
                    >
                      {markdownLink}
                    </SyntaxHighlighter>

                    <ButtonCopyToClipboard
                      text={markdownLink}
                      label="Copy badge markdown link to clipboard"
                      wrapperClassName="ms-3"
                    />
                  </div>
                </>
              ),
            },
            {
              name: 'ascii',
              title: 'AsciiDoc',
              content: (
                <>
                  <div className="mt-2 mb-4">
                    <img src={badgeImage} alt="CloMonitor badge" />
                  </div>

                  <div className="d-flex flex-row align-items-center">
                    <SyntaxHighlighter
                      language="bash"
                      style={docco}
                      customStyle={{
                        backgroundColor: 'var(--color-black-10)',
                        color: 'var(--color-font)',
                        marginBottom: '0',
                      }}
                    >
                      {asciiLink}
                    </SyntaxHighlighter>
                    <ButtonCopyToClipboard
                      text={asciiLink}
                      label="Copy badge Ascii link to clipboard"
                      wrapperClassName="ms-3"
                    />
                  </div>
                </>
              ),
            },
          ]}
          active="markdown"
          noDataContent="Sorry, the information for this is missing."
        />
      </div>
    </Modal>
  );
};

export default BadgeModal;
