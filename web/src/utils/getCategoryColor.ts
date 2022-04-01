import { isUndefined } from 'lodash';

const getCategoryColor = (value?: number): string => {
  if (isUndefined(value)) return '';
  if (value < 25) {
    return 'red';
  } else if (value >= 25 && value < 50) {
    return 'orange';
  } else if (value >= 50 && value < 75) {
    return 'yellow';
  }
  return 'green';
};

export default getCategoryColor;
