import scrollToPosition from './scrollToPosition';

describe('scrollToPosition', () => {
  const originalScrollBehavior = document.documentElement.style.scrollBehavior;
  const originalRequestAnimationFrame = window.requestAnimationFrame;

  afterEach(() => {
    jest.restoreAllMocks();
    document.documentElement.style.scrollBehavior = originalScrollBehavior;
    window.requestAnimationFrame = originalRequestAnimationFrame;
    jest.useRealTimers();
  });

  it('restores previous scroll behavior when requestAnimationFrame runs', () => {
    const scrollingElement = document.documentElement;
    scrollingElement.style.scrollBehavior = 'smooth';
    const scrollSpy = jest.spyOn(window, 'scrollTo').mockImplementation(() => undefined);
    jest
      .spyOn(window, 'requestAnimationFrame')
      .mockImplementation((callback: FrameRequestCallback) => (callback(0), 1));

    scrollToPosition(240);

    expect(scrollSpy).toHaveBeenCalledWith(expect.objectContaining({ top: 240, left: 0, behavior: 'auto' }));
    expect(scrollingElement.style.scrollBehavior).toBe('smooth');
  });

  it('falls back to setTimeout when requestAnimationFrame is not available', () => {
    jest.useFakeTimers();
    const scrollingElement = document.documentElement;
    scrollingElement.style.removeProperty('scroll-behavior');
    const scrollSpy = jest.spyOn(window, 'scrollTo').mockImplementation(() => undefined);
    // @ts-expect-error - intentionally unset for test
    window.requestAnimationFrame = undefined;

    scrollToPosition(120);

    expect(scrollSpy).toHaveBeenCalledWith(expect.objectContaining({ top: 120, left: 0, behavior: 'auto' }));
    expect(scrollingElement.style.scrollBehavior).toBe('auto');
    jest.runAllTimers();
    expect(scrollingElement.style.scrollBehavior).toBe('');
  });
});
