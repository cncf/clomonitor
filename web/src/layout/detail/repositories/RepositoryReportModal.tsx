import { BlockCodeButtons } from 'clo-ui/components/BlockCodeButtons';
import { Loading } from 'clo-ui/components/Loading';
import { Modal } from 'clo-ui/components/Modal';
import { isUndefined } from 'lodash';
import { useContext, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';

import API from '../../../api';
import { AppContext } from '../../../context/AppContextProvider';
import { CodeBlock } from '../../common/CodeBlock';
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
          <div className="border border-1 overflow-auto h-100 mw-100">
            <div className={`position-relative h-100 mh-100 ${styles.syntaxWrapper}`}>
              {isGettingMd && <Loading />}
              <BlockCodeButtons
                filename={`${props.repoName}-report.md`}
                content={report || ''}
                className="mt-2"
                hiddenDownloadBtn
              />

              <CodeBlock
                language="markdown"
                content={report || ''}
                withCopyBtn={false}
                style={{
                  height: '100%',
                  fontSize: '0.8rem',
                }}
                effective_theme={effective}
              />
            </div>
          </div>
        </Modal>
      )}
    </>
  );
};

export default RepositoryReportModal;
