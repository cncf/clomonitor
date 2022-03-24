const updateMetaIndex = (
  title: string = 'CLOMonitor',
  description: string = 'CLOMonitor is a tool that periodically checks CNCF projects repositories to verify they meet a certain project health best practices.'
): void => {
  document.title = title;
  document.querySelector(`meta[property='og:title']`)!.setAttribute('content', title);
  document.querySelector(`meta[name='twitter:title']`)!.setAttribute('content', title);
  document.querySelector(`meta[name='description']`)!.setAttribute('content', description);
  document.querySelector(`meta[property='og:description']`)!.setAttribute('content', description);
  document.querySelector(`meta[name='twitter:description']`)!.setAttribute('content', description);
};

export default updateMetaIndex;
