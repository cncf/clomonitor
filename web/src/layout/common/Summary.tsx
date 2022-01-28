import { BiLock } from 'react-icons/bi';
import { GoLaw } from 'react-icons/go';
import { HiCode, HiOutlinePencilAlt } from 'react-icons/hi';

import { Score } from '../../types';
import getCategoryColor from '../../utils/getCategoryColor';
import Category from './Category';
import styles from './Summary.module.css';

interface Props {
  score: Score;
  noBg?: boolean;
  bigSize?: boolean;
}

const SummaryCard = (props: Props) => {
  const color = getCategoryColor(props.score.global);

  return (
    <div
      className={`d-flex flex-column flex-md-row flex-lg-column flex-xl-row align-items-center ${
        props.noBg ? '' : `${styles.summary} p-3`
      }`}
    >
      <div className="d-none d-md-block d-lg-none d-xl-block d-lg-none d-xl-block">
        <div
          style={{ borderColor: `var(--rm-${color})` }}
          className={`d-flex align-items-center justify-content-center fs-2 me-0 me-sm-4 mb-2 mb-sm-0 rounded-pill ${styles.value} global`}
        >
          {props.score.global}
        </div>
      </div>
      <div className="d-block d-md-none d-lg-block d-xl-none w-100">
        <div className="mb-3 justify-content-center row">
          <Category name="Global" value={props.score.license} bigSize={props.bigSize} colNumber={8} />
        </div>
      </div>

      <div className={`flex-grow-1 w-100 position-relative ${styles.categories}`}>
        <div className="row">
          <Category
            name="Documentation"
            shortName="Docs"
            value={props.score.documentation}
            icon={<HiOutlinePencilAlt />}
            bigSize={props.bigSize}
          />
          <Category name="License" value={props.score.license} icon={<GoLaw />} bigSize={props.bigSize} />
        </div>
        <div className="row mt-1">
          <Category
            name="Code quality"
            shortName="Quality"
            value={props.score.quality}
            icon={<HiCode />}
            bigSize={props.bigSize}
          />
          <Category name="Security" value={props.score.security} icon={<BiLock />} bigSize={props.bigSize} />
        </div>
      </div>
    </div>
  );
};

export default SummaryCard;
