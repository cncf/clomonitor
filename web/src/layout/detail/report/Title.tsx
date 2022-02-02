interface Props {
  title: string;
  icon: JSX.Element;
}

const Title = (props: Props) => {
  return (
    <div className="d-flex flex-row align-items-center">
      {props.icon}
      <div className="ms-2 fw-bold">{props.title}</div>
    </div>
  );
};

export default Title;
