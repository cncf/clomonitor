import { isUndefined } from 'lodash';
import { useLocation, useNavigate } from 'react-router-dom';

import getCategoryColor from '../../utils/getCategoryColor';
import roundScoreValue from '../../utils/roundScoreValue';
import styles from './CategoryProgressbar.module.css';

interface Props {
  icon?: JSX.Element;
  value?: number;
  name: string;
  bigSize?: boolean;
  linkTo?: string;
  scrollIntoView?: (id?: string) => void;
}

const CategoryProgressbar = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();
  const color = getCategoryColor(props.value);

  return (
    <div className={`${styles.wrapper} ${props.bigSize ? 'col-12 col-lg-9 col-xxxl-8' : 'col-12'}`}>
      <div className="d-flex flex-row bg-white position-relative border overflow-hidden">
        <div
          className={`d-flex flex-row align-items-center text-muted fw-bold flex-nowrap px-1 my-auto ${styles.title}`}
        >
          {props.icon && <span className={`pe-1 d-inline-block position-relative ${styles.icon}`}>{props.icon}</span>}
          {!isUndefined(props.linkTo) ? (
            <button
              className={`btn btn-link text-truncate text-muted fw-bold p-0 text-decoration-none ${styles.btn}`}
              onClick={() => {
                if (props.scrollIntoView) {
                  props.scrollIntoView(`#${props.linkTo}`);
                }
                navigate(
                  {
                    pathname: location.pathname,
                    hash: props.linkTo,
                  },
                  { state: location.state }
                );
              }}
              aria-label={`Go from summary to section: ${props.linkTo}`}
            >
              {props.name}
            </button>
          ) : (
            <span className="text-truncate">{props.name}</span>
          )}
        </div>
        <div className={`text-center fw-bold font-monospace ${styles.value} ${props.bigSize ? styles.bigSize : ''}`}>
          {isUndefined(props.value) ? 'n/a' : roundScoreValue(props.value)}
        </div>
        <div
          className={`flex-grow-1 position-relative mx-2 ${styles.progressWrapper}  ${
            props.bigSize ? styles.progressBigWrapper : ''
          }`}
        >
          {!isUndefined(props.value) && (
            <div
              data-testid="line"
              className={`position-absolute start-0 top-0 bottom-0 ${styles.line}`}
              style={{
                width: `${props.value || 1}%`,
                backgroundColor: `var(--rm-${color})`,
              }}
            />
          )}
        </div>
      </div>
    </div>
  );
};

export default CategoryProgressbar;
