interface Props {
  title: string;
  icon: JSX.Element;
  className?: string;
  anchor?: JSX.Element;
  extra?: JSX.Element;
}

const Title = (props: Props) => {
  return (
    <div className={`d-flex flex-row align-items-center ${props.className}`}>
      {props.icon}
      <div className="ms-2 fw-bold">{props.title}</div>
      {props.extra}
      {props.anchor}
    </div>
  );
};

export default Title;
