import styles from './QualityDot.module.css';

interface Props {
  level: number;
  className?: string;
}

const QualityDot = (props: Props) => {
  return (
    <div
      data-testid="quality-dot"
      className={`me-2 border border-1 rounded-pill position-relative ${styles.quality} ${
        styles[`level${props.level}`]
      } ${props.className}`}
    />
  );
};

export default QualityDot;
