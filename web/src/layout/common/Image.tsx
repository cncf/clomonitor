import { isUndefined } from 'lodash';
import isNull from 'lodash/isNull';
import { useRef, useState } from 'react';
import { MdOutlineImageNotSupported } from 'react-icons/md';

interface Props {
  url?: string | null;
  alt: string;
  className?: string;
}

const Image = (props: Props) => {
  const image = useRef<HTMLImageElement | null>(null);
  const [error, setError] = useState(false);

  return (
    <>
      {error || isNull(props.url) || isUndefined(props.url) ? (
        <MdOutlineImageNotSupported data-testid="img-placeholder" />
      ) : (
        <img
          ref={image}
          alt={props.alt}
          src={props.url}
          className={props.className}
          onError={() => setError(true)}
          aria-hidden="true"
        />
      )}
    </>
  );
};

export default Image;
