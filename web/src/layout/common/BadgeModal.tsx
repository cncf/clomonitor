import { CodeBlock, Foundation, Modal, Tabs } from 'clo-ui';
import { useContext } from 'react';

import { AppContext } from '../../context/AppContextProvider';

interface OpenModalStatus {
  status: boolean;
  name?: string;
}

interface Props {
  foundation: Foundation;
  projectName: string;
  openStatus: OpenModalStatus;
  onCloseModal: () => void;
}

const BadgeModal = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const origin = window.location.origin;
  const badgeImage = `https://img.shields.io/endpoint?url=${origin}/api/projects/${props.foundation}/${props.projectName}/badge`;
  const markdownLink = `[![CLOMonitor](${badgeImage})](${origin}/projects/${props.foundation}/${props.projectName})`;
  const asciiLink = `${origin}/projects/${props.foundation}/${props.projectName}[image:${badgeImage}[CLOMonitor]]`;

  return (
    <Modal
      header="Project badge"
      onClose={props.onCloseModal}
      open={props.openStatus.status && props.openStatus.name === 'badge'}
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
                    <img src={badgeImage} alt="CLOMonitor badge" />
                  </div>

                  <CodeBlock
                    language="markdown"
                    content={markdownLink}
                    label="Copy badge markdown link to clipboard"
                    effective_theme={effective}
                    withCopyBtn
                  />
                </>
              ),
            },
            {
              name: 'ascii',
              title: 'AsciiDoc',
              content: (
                <>
                  <div className="mt-2 mb-4">
                    <img src={badgeImage} alt="CLOMonitor badge" />
                  </div>

                  <CodeBlock
                    language="asciidoc"
                    content={asciiLink}
                    label="Copy badge Ascii link to clipboard"
                    effective_theme={effective}
                    withCopyBtn
                  />
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
