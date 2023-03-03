import { ThemeMode as ThemeModeForm } from 'clo-ui';
import { isUndefined } from 'lodash';
import { useContext } from 'react';

import { AppContext, updateTheme } from '../../context/AppContextProvider';

interface Props {
  closeDropdown?: () => void;
  device: string;
}

const ThemeMode = (props: Props) => {
  const { ctx, dispatch } = useContext(AppContext);
  const { configured } = ctx.prefs.theme;

  const onHandleChange = (value: string) => {
    dispatch(updateTheme(value));
    if (!isUndefined(props.closeDropdown)) {
      props.closeDropdown();
    }
  };

  return <ThemeModeForm device={props.device} configuredTheme={configured} onChange={onHandleChange} />;
};

export default ThemeMode;
