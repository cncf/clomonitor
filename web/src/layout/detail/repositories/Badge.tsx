import { isUndefined } from 'lodash';
import { useLocation, useNavigate } from 'react-router-dom';

import getCategoryColor from '../../../utils/getCategoryColor';
import styles from './Badge.module.css';

interface Props {
  value?: number;
  linkTo: string;
  scrollIntoView: (id?: string) => void;
}

const Badge = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();

  if (isUndefined(props.value))
    return (
      <div className="mx-auto px-2 text-center">
        <span className="text-muted">n/a</span>
      </div>
    );

  const color = getCategoryColor(props.value);

  return (
    <div
      className={`mx-auto px-2 text-center ${styles.badge}`}
      style={{
        borderBottomColor: `var(--rm-${color})`,
      }}
    >
      <button
        className={`btn btn-link text-dark fw-bold p-0 text-decoration-none ${styles.btn}`}
        onClick={() => {
          props.scrollIntoView(`#${props.linkTo}`);
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
        {props.value}
      </button>
    </div>
  );
};

export default Badge;
