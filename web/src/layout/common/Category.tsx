import classNames from 'classnames';
import { isUndefined } from 'lodash';

import getCategoryColor from '../../utils/getCategoryColor';
import styles from './Category.module.css';

interface Props {
  icon?: JSX.Element;
  value: number;
  name: string;
  shortName?: string;
  bigSize?: boolean;
  colNumber?: number;
}

const Category = (props: Props) => {
  const color = getCategoryColor(props.value);
  return (
    <div className={`col-${props.colNumber || 6} text-truncate`}>
      <small className={`text-muted fw-bold text-nowrap ${styles.title}`}>
        {props.icon && <span className={`pe-1 d-inline-block position-relative ${styles.icon}`}>{props.icon}</span>}
        <span
          className={classNames({
            'd-none d-md-inline-block': !isUndefined(props.shortName),
          })}
        >
          {props.name}
        </span>
        {!isUndefined(props.shortName) && <span className="d-inline-block d-md-none">{props.shortName}</span>}
      </small>
      <div className={`d-flex flex-row bg-white position-relative border overflow-hidden ${styles.line}`}>
        <div className={`text-center fw-bold font-monospace ${styles.value} ${props.bigSize ? styles.bigSize : ''}`}>
          {props.value}
        </div>
        <div className="flex-grow-1 w-100 position-relative">
          <div
            className="position-absolute top-0 start-0 bottom-0"
            style={{
              width: props.value === 100 ? '100%' : `calc(${props.value}% - 5px)`,
              backgroundColor: `var(--rm-${color})`,
            }}
          >
            {props.value !== 100 && (
              <div
                style={{ borderLeftColor: `var(--rm-${color})` }}
                className={`position-absolute ${styles.arrow} ${props.bigSize ? styles.bigArrow : ''}`}
              />
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Category;
