import React from 'react';

import { CheckSet } from '../../../types';
import styles from './Badge.module.css';

interface Props {
  checkSets: CheckSet[];
  className?: string;
}

const CheckSetBadge = (props: Props) => {
  return (
    <div className={`d-inline-flex ${props.className}`}>
      <div
        data-testid="repo-kind-badge"
        className={`badge text-secondary border border-secondary rounded-0 position-relative text-uppercase ${styles.miniBadge} ${props.className}`}
      >
        <div className="d-flex flex-row align-items-center">
          {props.checkSets.map((k: CheckSet, index: number) => {
            return (
              <React.Fragment key={`kind_${k}`}>
                {index !== 0 && <div className={`px-1 position-relative ${styles.symbol}`}>+</div>}
                {k}
              </React.Fragment>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default CheckSetBadge;
