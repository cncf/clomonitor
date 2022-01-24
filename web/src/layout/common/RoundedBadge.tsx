import styles from './RoundedBadge.module.css';

interface Props {
  level: number;
  className?: string;
}

const RoundedBadge = (props: Props) => {
  return (
    <div
      className={`me-2 border rounded-pill position-relative ${styles.quality} ${styles[`level${props.level}`]} ${
        props.className
      }`}
    />
  );
};

export default RoundedBadge;
