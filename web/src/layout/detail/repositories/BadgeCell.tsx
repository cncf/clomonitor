import classNames from 'classnames';
import { isUndefined } from 'lodash';

import getCategoryColor from '../../../utils/getCategoryColor';
import roundScoreValue from '../../../utils/roundScoreValue';
import styles from './BadgeCell.module.css';

interface Props {
  value?: number;
  cellClassName?: string;
  onClick: () => void;
}

const BadgeCell = (props: Props) => {
  if (isUndefined(props.value))
    return (
      <td className={props.cellClassName || 'd-none d-md-table-cell align-middle'}>
        <div className="mx-auto px-2 text-center">
          <span className="text-muted">n/a</span>
        </div>
      </td>
    );

  const color = getCategoryColor(props.value);

  return (
    <td
      className={classNames(
        styles.hoverableCell,
        { 'd-none d-md-table-cell align-middle': isUndefined(props.cellClassName) },
        props.cellClassName
      )}
      onClick={props.onClick}
      role="button"
    >
      <div
        className={`mx-auto px-2 text-center text-dark fw-bold ${styles.badge}`}
        style={{
          borderBottomColor: `var(--rm-${color})`,
        }}
      >
        {roundScoreValue(props.value)}
      </div>
    </td>
  );
};

export default BadgeCell;
