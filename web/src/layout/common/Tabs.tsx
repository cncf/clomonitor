import classnames from 'classnames';
import { memo, MouseEvent as ReactMouseEvent, useEffect, useState } from 'react';

import NoData from '../common/NoData';
import styles from './Tabs.module.css';

interface Props {
  tabs: Tab[];
  active: string;
  noDataContent: string;
  className?: string;
}

interface Tab {
  name: string;
  title: string;
  shortTitle?: string;
  content: JSX.Element;
}

const Tabs = (props: Props) => {
  const [activeTab, setActiveTab] = useState(props.active);
  const [visibleContent, setVisibleContent] = useState<JSX.Element | undefined>();

  useEffect(() => {
    const currentActiveTab = props.tabs.find((tab: Tab) => tab.name === activeTab);
    if (currentActiveTab) {
      setVisibleContent(currentActiveTab.content);
    }
  }, [props.tabs, activeTab]);

  return (
    <>
      <div className={props.className}>
        <ul className={`nav nav-tabs ${styles.tabs}`}>
          {props.tabs.map((tab: Tab) => (
            <li className="nav-item" key={tab.name}>
              <button
                className={classnames('btn nav-item rounded-0 lightText', styles.btn, {
                  [`active btn-secondary ${styles.active}`]: tab.name === activeTab,
                })}
                onClick={(e: ReactMouseEvent<HTMLButtonElement, MouseEvent>) => {
                  e.preventDefault();
                  e.stopPropagation();
                  setActiveTab(tab.name);
                  setVisibleContent(tab.content);
                }}
                aria-label={`Open tab ${tab.name}`}
              >
                <span className="d-none d-sm-block">{tab.title}</span>
                <span className="d-block d-sm-none">{tab.shortTitle || tab.title}</span>
              </button>
            </li>
          ))}
        </ul>
      </div>

      <div className="tab-content mt-4">
        <div className="tab-pane fade show active">
          {visibleContent ? <>{visibleContent}</> : <NoData>{props.noDataContent}</NoData>}
        </div>
      </div>
    </>
  );
};

export default memo(Tabs);
