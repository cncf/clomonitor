const getMetaTag = (name: string, isTrue?: boolean): any => {
  const value = document.querySelector(`meta[name='remonitor:${name}']`)
    ? document.querySelector(`meta[name='remonitor:${name}']`)!.getAttribute('content')
    : null;
  if (isTrue) {
    return value === 'true';
  } else {
    return value;
  }
};

export default getMetaTag;
