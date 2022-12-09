import { isUndefined } from 'lodash';
import { useContext, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import SyntaxHighlighter from 'react-syntax-highlighter';
import { docco } from 'react-syntax-highlighter/dist/cjs/styles/hljs';
import { tomorrowNight } from 'react-syntax-highlighter/dist/cjs/styles/hljs';

import API from '../../../api';
import { AppContext } from '../../../context/AppContextProvider';
import BlockCodeButtons from '../../common/BlockCodeButtons';
import Loading from '../../common/Loading';
import Modal from '../../common/Modal';
import styles from './RepositoryReportModal.module.css';

interface Props {
  openStatus: boolean;
  repoName: string;
  onCloseModal: () => void;
}

const RepositoryReportModal = (props: Props) => {
  const { project, foundation } = useParams();
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const [report, setReport] = useState<string | undefined>();
  const [isGettingMd, setIsGettingMd] = useState<boolean>(false);

  useEffect(() => {
    async function getReportMd() {
      try {
        setIsGettingMd(true);
        setReport(await API.getRepositoryReportMD(foundation!, project!, props.repoName));
        setIsGettingMd(false);
      } catch {
        setReport('');
        setIsGettingMd(false);
      }
    }

    if (!isUndefined(foundation) && !isUndefined(project) && props.openStatus) {
      getReportMd();
    }
  }, [project, foundation, props.repoName, props.openStatus]);

  return (
    <>
      {props.openStatus && (
        <Modal
          modalDialogClassName={styles.modalDialog}
          modalClassName={styles.modalContent}
          header="Repository report"
          size="xl"
          onClose={props.onCloseModal}
          open={props.openStatus}
        >
          <div className="border overflow-auto h-100 mw-100">
            <div className={`position-relative h-100 mh-100 ${styles.syntaxWrapper}`}>
              {isGettingMd && <Loading />}
              <BlockCodeButtons
                filename={`${props.repoName}-report.md`}
                content={report || ''}
                className="mt-2"
                hiddenDownloadBtn
              />

              <SyntaxHighlighter
                language="markdown"
                style={effective === 'dark' ? tomorrowNight : docco}
                customStyle={{
                  backgroundColor: 'var(--bg-code)',
                  color: 'var(--color-font)',
                  padding: '1rem',
                  marginBottom: 0,
                  fontSize: '0.8rem',
                  height: '100%',
                }}
              >
                {report || ''}
              </SyntaxHighlighter>
            </div>
          </div>
        </Modal>
      )}
    </>
  );
};

export default RepositoryReportModal;
