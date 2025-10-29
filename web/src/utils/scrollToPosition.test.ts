import { vi } from 'vitest';

import scrollToPosition from './scrollToPosition';

describe('scrollToPosition', () => {
  const originalScrollBehavior = document.documentElement.style.scrollBehavior;
  const originalRequestAnimationFrame = window.requestAnimationFrame;

  afterEach(() => {
    vi.restoreAllMocks();
    document.documentElement.style.scrollBehavior = originalScrollBehavior;
    window.requestAnimationFrame = originalRequestAnimationFrame;
    vi.useRealTimers();
  });

  it('restores previous scroll behavior when requestAnimationFrame runs', () => {
    const scrollingElement = document.documentElement;
    scrollingElement.style.scrollBehavior = 'smooth';
    const scrollSpy = vi.spyOn(window, 'scrollTo').mockImplementation(() => undefined);
    vi.spyOn(window, 'requestAnimationFrame').mockImplementation((callback: FrameRequestCallback) => (callback(0), 1));

    scrollToPosition(240);

    expect(scrollSpy).toHaveBeenCalledWith(expect.objectContaining({ top: 240, left: 0, behavior: 'auto' }));
    expect(scrollingElement.style.scrollBehavior).toBe('smooth');
  });

  it('falls back to setTimeout when requestAnimationFrame is not available', () => {
    vi.useFakeTimers();
    const scrollingElement = document.documentElement;
    scrollingElement.style.removeProperty('scroll-behavior');
    const scrollSpy = vi.spyOn(window, 'scrollTo').mockImplementation(() => undefined);
    // @ts-expect-error - intentionally unset for test
    window.requestAnimationFrame = undefined;

    scrollToPosition(120);

    expect(scrollSpy).toHaveBeenCalledWith(expect.objectContaining({ top: 120, left: 0, behavior: 'auto' }));
    expect(scrollingElement.style.scrollBehavior).toBe('auto');
    vi.runAllTimers();
    expect(scrollingElement.style.scrollBehavior).toBe('');
  });
});
