import { REPORT_OPTIONS } from '../../data';
import { ReportOption, ReportOptionData } from '../../types';
import sortChecks from '../../utils/sortChecks';
import ProgressBarInLine from './ProgressBarInLine';

interface Props {
  title: string;
  data: { [key in ReportOption]?: number };
  onSelectCheck: (name: ReportOption) => void;
}

function getOptionInfo(key: ReportOption) {
  return REPORT_OPTIONS[key];
}

const Checks = (props: Props) => {
  const sortedChecks = sortChecks(props.data);

  return (
    <div className="col-12">
      <div className="fw-bold border-bottom pb-2 mb-3 lightText">{props.title}</div>
      <div className="mx-1 pt-1">
        {sortedChecks.map((check: ReportOption) => {
          const opt: ReportOptionData = getOptionInfo(check);
          return (
            <ProgressBarInLine
              key={`check_${opt.name}`}
              title={opt.shortName || opt.name}
              name={check}
              icon={opt.icon}
              value={props.data[check] as number}
              onSelectCheck={props.onSelectCheck}
            />
          );
        })}
      </div>
    </div>
  );
};

export default Checks;
