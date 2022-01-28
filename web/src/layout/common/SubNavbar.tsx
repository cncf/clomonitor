import styles from './SubNavbar.module.css';

interface Props {
  children: JSX.Element;
}

const SubNavbar = (props: Props) => {
  return (
    <nav className={`navbar navbar-expand-sm ${styles.navbar}`} role="navigation">
      <div className="container-lg">{props.children}</div>
    </nav>
  );
};

export default SubNavbar;
