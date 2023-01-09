import { isUndefined } from 'lodash';
import isNull from 'lodash/isNull';
import { useContext, useRef, useState } from 'react';
import { MdOutlineImageNotSupported } from 'react-icons/md';

import { AppContext } from '../../context/AppContextProvider';

interface Props {
  url?: string | null;
  dark_url?: string | null;
  alt: string;
  className?: string;
}

const Image = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
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
          src={effective === 'dark' ? props.dark_url || props.url : props.url}
          className={props.className}
          onError={() => setError(true)}
          aria-hidden="true"
        />
      )}
    </>
  );
};

export default Image;
