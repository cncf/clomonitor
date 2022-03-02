import getCategoryColor from '../../utils/getCategoryColor';
import styles from './CategoryProgressbar.module.css';

interface Props {
  icon?: JSX.Element;
  value: number;
  name: string;
  bigSize?: boolean;
}

const CategoryProgressbar = (props: Props) => {
  const color = getCategoryColor(props.value);
  return (
    <div className={`${styles.wrapper} ${props.bigSize ? 'col-12 col-lg-9 col-xxxl-8' : 'col-12'}`}>
      <div className={`d-flex flex-row bg-white position-relative border overflow-hidden ${styles.line}`}>
        <div
          className={`d-flex flex-row align-items-center text-muted fw-bold flex-nowrap px-1 my-auto ${styles.title}`}
        >
          {props.icon && <span className={`pe-1 d-inline-block position-relative ${styles.icon}`}>{props.icon}</span>}
          <span className="text-truncate">{props.name}</span>
        </div>
        <div className={`text-center fw-bold font-monospace ${styles.value} ${props.bigSize ? styles.bigSize : ''}`}>
          {props.value}
        </div>
        <div className="flex-grow-1 position-relative">
          <div
            data-testid="line"
            className="position-absolute top-0 start-0 bottom-0"
            style={{
              width: `calc(${props.value}% - 5px)`,
              backgroundColor: `var(--rm-${color})`,
            }}
          >
            <div
              data-testid="peak"
              style={{ borderLeftColor: `var(--rm-${color})` }}
              className={`position-absolute ${styles.arrow} ${props.bigSize ? styles.bigArrow : ''}`}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default CategoryProgressbar;
