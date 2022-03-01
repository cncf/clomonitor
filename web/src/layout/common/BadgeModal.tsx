import CodeBlock from './CodeBlock';
import Modal from './Modal';
import Tabs from './Tabs';

interface OpenModalStatus {
  status: boolean;
  name?: string;
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
  const markdownLink = `[![CLOMonitor](${badgeImage})](${origin}/projects/${props.orgName}/${props.projectName})`;
  const asciiLink = `${origin}/projects/${props.orgName}/${props.projectName}[image:${badgeImage}[CLOMonitor]]`;

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

                  <CodeBlock language="markdown" content={markdownLink} label="Copy badge markdown link to clipboard" />
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

                  <CodeBlock language="asciidoc" content={asciiLink} label="Copy badge Ascii link to clipboard" />
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
