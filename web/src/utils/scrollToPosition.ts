// Forces an immediate scroll jump regardless of smooth-scroll styles.
const scrollToPosition = (value?: number) => {
  if (typeof window === 'undefined') {
    return;
  }

  const top = value ?? 0;
  const scrollingElement = (document.scrollingElement || document.documentElement) as HTMLElement | null;

  if (scrollingElement) {
    const previousBehavior = scrollingElement.style.scrollBehavior;
    scrollingElement.style.scrollBehavior = 'auto';
    window.scrollTo({ top, left: 0, behavior: 'auto' });
    const restoreBehavior = () => {
      if (previousBehavior) {
        scrollingElement.style.scrollBehavior = previousBehavior;
      } else {
        scrollingElement.style.removeProperty('scroll-behavior');
      }
    };

    if (typeof window.requestAnimationFrame === 'function') {
      window.requestAnimationFrame(restoreBehavior);
    } else {
      window.setTimeout(restoreBehavior, 0);
    }
    return;
  }

  window.scrollTo({ top, left: 0, behavior: 'auto' });
};

export default scrollToPosition;
