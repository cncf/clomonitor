(() => {
  const updateActiveStyleSheet = (currentTheme) => {
    document.getElementsByTagName('html')[0].setAttribute('data-theme', currentTheme);
    document
      .querySelector(`meta[name='theme-color']`)
      .setAttribute('content', currentTheme === 'light' ? '#2a0552' : '#131216');
  };

  let theme = 'light'; // By default, light theme is enabled
  const activeProfile = 'guest'; // We are only using guest at this moment
  const prefs = localStorage.getItem('clomonitorPrefs');
  if (activeProfile && prefs) {
    const savedPrefs = JSON.parse(prefs);
    const activeUserPrefs = savedPrefs[activeProfile];
    if (activeUserPrefs) {
      theme = activeUserPrefs.theme.effective;
    }
  }
  updateActiveStyleSheet(theme);
})();
