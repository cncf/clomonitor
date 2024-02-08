import classNames from 'classnames';
import { alertDispatcher, Foundation, Loading, Maturity, NoData, prettifyNumber, SubNavbar, Timeline } from 'clo-ui';
import { groupBy, isEmpty, isNull, isNumber, isUndefined, range } from 'lodash';
import moment from 'moment';
import { ChangeEvent, useCallback, useContext, useEffect, useState } from 'react';
import ReactApexChart from 'react-apexcharts';
import { GrDocumentCsv } from 'react-icons/gr';
import { useLocation, useNavigate, useSearchParams } from 'react-router-dom';

import API from '../../api';
import { AppContext } from '../../context/AppContextProvider';
import { FOUNDATIONS } from '../../data';
import { DistributionData, FilterKind, Rating, RatingKind, ReportOption, Stats } from '../../types';
import prepareQueryString from '../../utils/prepareQueryString';
import AnchorHeader from './AnchorHeader';
import Average from './Average';
import Checks from './Checks';
import styles from './StatsView.module.css';

interface HeatMapData {
  name: string;
  data: number[];
}

interface SelectedPoint {
  rating: string[];
  maturity?: string[];
}

interface SelectedRange {
  from: string;
  to: string;
}

const FOUNDATION_QUERY = 'foundation';

const StatsView = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const [searchParams, setSearchParams] = useSearchParams();
  const [isLightActive, setIsLightActive] = useState<boolean>(effective === 'light');
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [emptyStats, setEmptyStats] = useState<boolean>(false);
  const [stats, setStats] = useState<Stats | null | undefined>();
  const [apiError, setApiError] = useState<string | null>(null);
  const selectedFoundation = searchParams.get(FOUNDATION_QUERY);
  const [selectedPoint, setSelectedPoint] = useState<SelectedPoint | undefined>();
  const [selectedRange, setSelectedRange] = useState<SelectedRange | undefined>();
  const [downloadingCSV, setDownloadingCSV] = useState<boolean>(false);
  const [activeDate, setActiveDate] = useState<string | undefined>();
  const [snapshots, setSnapshots] = useState<string[] | undefined>(undefined);

  useEffect(() => {
    setIsLightActive(effective === 'light');
  }, [effective]);

  const scrollIntoView = useCallback(
    (id?: string) => {
      const elId = id || location.hash;
      if (isUndefined(elId) || elId === '') return;
      try {
        const element = document.getElementById(elId.replace('#', ''));
        if (element) {
          element.scrollIntoView({ block: 'start', inline: 'nearest', behavior: 'smooth' });
        }
      } finally {
        return;
      }
    },
    [location.hash]
  );

  const handleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value;
    if (value === '') {
      searchParams.delete(FOUNDATION_QUERY);
    } else {
      searchParams.set(FOUNDATION_QUERY, value);
    }
    setSearchParams(searchParams);
  };

  const checkCurrentStats = (currentStats: Stats | null) => {
    if (!isNull(currentStats)) {
      const notEmptyItems = Object.keys(currentStats).some((elem: string) => {
        return elem !== 'generated_at' && !isEmpty((currentStats as any)[elem]);
      });
      setEmptyStats(!notEmptyItems);
    }
  };

  const selectCheck = (name: ReportOption) => {
    navigate(
      {
        pathname: '/search',
        search: prepareQueryString({
          filters: {
            [FilterKind.PassingCheck]: [name],
            ...(!isNull(selectedFoundation) ? { [FOUNDATION_QUERY]: [selectedFoundation] } : {}),
          },
          pageNumber: 1,
        }),
      },
      { state: { resetScrollPosition: true } }
    );
  };

  const loadSearchPage = (filters: SelectedPoint) => {
    navigate(
      {
        pathname: '/search',
        search: prepareQueryString({
          filters: {
            ...filters,
            ...(!isNull(selectedFoundation) ? { [FOUNDATION_QUERY]: [selectedFoundation] } : {}),
          },
          pageNumber: 1,
        }),
      },
      { state: { resetScrollPosition: true } }
    );
  };

  const prepareMonthlyViewsData = (data: number[][]): number[][] => {
    let finalData: number[][] = [...data];
    if (data.length < 13 && data.length > 0) {
      const oldDate = data[0][0];
      range(1, 13 - data.length).forEach((x: number) => {
        finalData.unshift([
          moment
            .unix(oldDate / 1000)
            .subtract(x, 'month')
            .unix() * 1000,
          0,
        ]);
      });
    }
    return finalData;
  };

  const loadSearchPageWithAcceptedRange = (range: SelectedRange) => {
    navigate(
      {
        pathname: '/search',
        search: prepareQueryString({
          accepted_from: range.from,
          accepted_to: range.to,
          filters: {
            ...(!isNull(selectedFoundation) ? { [FOUNDATION_QUERY]: [selectedFoundation] } : {}),
          },
          pageNumber: 1,
        }),
      },
      { state: { resetScrollPosition: true } }
    );
  };

  const getAreaChartConfig = (): ApexCharts.ApexOptions => {
    return {
      chart: {
        fontFamily: "'Lato', Roboto, 'Helvetica Neue', Arial, sans-serif !default",
        height: 250,
        type: 'area',
        redrawOnWindowResize: true,
        redrawOnParentResize: true,
        // Temporary solution -> https://github.com/apexcharts/apexcharts.js/issues/4154 and https://github.com/apexcharts/Blazor-ApexCharts/issues/376
        animations: {
          enabled: false,
        },
        zoom: {
          type: 'x',
          enabled: true,
          autoScaleYaxis: true,
          zoomedArea: {
            fill: {
              color: 'var(--clo-secondary-15)',
              opacity: 0.4,
            },
            stroke: {
              color: 'var(--clo-secondary-900)',
              opacity: 0.8,
              width: 1,
            },
          },
        },
        toolbar: {
          autoSelected: 'zoom',
          tools: {
            download: false,
            pan: false,
          },
        },
        events: {
          beforeZoom: (chartContext: any, opt: any) => {
            const minDate = chartContext.ctx.data.twoDSeriesX[0];
            const maxDate = chartContext.ctx.data.twoDSeriesX[chartContext.ctx.data.twoDSeriesX.length - 1];
            let newMinDate = parseInt(opt.xaxis.min);
            let newMaxDate = parseInt(opt.xaxis.max);
            // Min range 1 week
            if (newMinDate > chartContext.minX) {
              const maxRange = moment(newMinDate).add(1, 'w').valueOf();
              if (moment(newMaxDate).isBefore(maxRange) && moment(maxRange).isBefore(maxDate)) {
                newMaxDate = maxRange;
              } else {
                const minRange = moment(newMaxDate).subtract(1, 'w').valueOf();
                if (moment(newMinDate).isAfter(minRange)) {
                  newMinDate = minRange;
                }
              }
            }
            return {
              xaxis: {
                min: newMinDate < minDate ? minDate : newMinDate,
                max: newMaxDate > maxDate ? maxDate : newMaxDate,
              },
            };
          },
        },
      },
      grid: { borderColor: 'var(--color-light-gray)' },
      dataLabels: {
        enabled: false,
      },
      colors: ['#90be6d'],
      stroke: {
        curve: 'smooth',
      },
      fill: {
        opacity: 0.5,
        colors: ['#90be6d'],
      },
      xaxis: {
        type: 'datetime',
        labels: {
          datetimeFormatter: {
            year: 'yyyy',
            month: "MMM'yy",
            day: 'dd MMM',
            hour: 'dd MMM',
          },
          style: {
            colors: 'var(--color-font)',
            fontSize: '11px',
          },
        },
      },
      yaxis: {
        labels: {
          style: {
            colors: ['var(--color-font)'],
          },
        },
      },
      markers: {
        size: 0,
      },
    };
  };

  const getDonutChartConfig = (maturityLevel?: string): ApexCharts.ApexOptions => {
    return {
      chart: {
        id: `${maturityLevel || 'all'}Donut`,
        fontFamily: "'Lato', Roboto, 'Helvetica Neue', Arial, sans-serif !default",
        height: 250,
        type: 'donut',
        redrawOnWindowResize: true,
        redrawOnParentResize: true,
        events: {
          dataPointSelection: (event: any, chartContext: any, config: any) => {
            setSelectedPoint({
              rating: [Object.values(Rating)[config.dataPointIndex]],
              ...(!isUndefined(maturityLevel) && { maturity: [maturityLevel] }),
            });
          },
        },
      },
      labels: Object.keys(RatingKind),
      dataLabels: {
        background: {
          enabled: true,
          foreColor: 'var(--bs-dark)',
          borderRadius: 0,
          borderColor: 'var(--solid-border)',
          opacity: 0.9,
          dropShadow: {
            enabled: false,
          },
        },
        dropShadow: { enabled: false },
      },
      stroke: { colors: isLightActive ? ['var(--bs-white)'] : ['#333'] },
      colors: ['#90be6d', '#f9c74f', '#f8961e', '#f94144'],
      legend: {
        position: 'bottom',
        labels: {
          colors: 'var(--color-font)',
        },
      },
      states: {
        hover: {
          filter: {
            type: 'darken',
            value: 0.8,
          },
        },
        active: {
          allowMultipleDataPointsSelection: false,
          filter: {
            type: 'none',
          },
        },
      },
      tooltip: {
        fillSeriesColor: false,
      },
    };
  };

  const getHeatMapChartConfig = (): ApexCharts.ApexOptions => {
    return {
      chart: {
        height: 250,
        type: 'heatmap',
        redrawOnWindowResize: true,
        redrawOnParentResize: true,
        toolbar: {
          show: false,
        },
        events: {
          dataPointSelection: (event: any, chartContext: any, config: any) => {
            const value = config.w.globals.series[config.seriesIndex][config.dataPointIndex] - 10;
            if (value > 0) {
              const selectedMonth = config.w.globals.labels[config.dataPointIndex];
              const selectedYear = config.w.globals.seriesNames[config.seriesIndex];
              const initialDay = `${selectedYear}-${selectedMonth}-01`;
              setSelectedRange({
                from: moment(initialDay, 'YYYY-MMM-DD').format('YYYY-MM-DD'),
                to: moment(initialDay, 'YYYY-MMM-DD').endOf('month').format('YYYY-MM-DD'),
              });
            }
          },
        },
      },
      grid: { borderColor: 'var(--color-light-gray)' },
      labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
      dataLabels: {
        enabled: false,
      },
      legend: { show: false },
      colors: ['#90be6d'],
      xaxis: {
        labels: {
          style: {
            colors: 'var(--color-font)',
            fontSize: '10px',
          },
        },
      },
      yaxis: {
        labels: {
          style: {
            colors: ['var(--color-font)'],
          },
        },
      },
      tooltip: {
        y: {
          formatter: (val: number): string => {
            // Subsctract 10 to display correct value
            return isNumber(val) ? (val - 10).toString() : val;
          },
        },
      },
      states: {
        hover: {
          filter: {
            type: 'darken',
            value: 0.8,
          },
        },
      },
      plotOptions: {
        heatmap: {
          radius: 0,
          shadeIntensity: 0,
          colorScale: {
            inverse: false,
            min: 0,
            max: 100,
            ranges: [
              {
                from: 0,
                to: 0,
                color: 'transparent',
              },
              {
                from: 1,
                to: 10,
                color: isLightActive ? '#f2f2f2' : '#424549',
              },
              {
                from: 11,
                to: 11,
                color: isLightActive ? '#d2e5c4' : '#2b4c2e',
              },
              {
                from: 12,
                to: 14,
                color: isLightActive ? '#90be6d' : '#4c8550',
              },
              {
                from: 15,
                to: 100,
                color: isLightActive ? '#567241' : '#6dbe73',
              },
            ],
          },
        },
      },
    };
  };

  const getBarChartConfig = (
    monthlyFormatter: boolean,
    dataLength: number,
    lastBarDate?: number
  ): ApexCharts.ApexOptions => {
    const getBarColors = (): string[] => {
      if (dataLength > 0) {
        const isCurrent = !isUndefined(lastBarDate)
          ? moment(moment.unix(lastBarDate / 1000)).isSame(moment(), monthlyFormatter ? 'month' : 'day')
          : false;
        let colors = Array.from({ length: dataLength - 1 }, () => 'var(--clo-tertiary)');
        if (isCurrent) {
          // Color for the last bar
          colors.push(effective === 'dark' ? '#cce7ff' : '#003666');
        } else {
          colors.push('var(--clo-tertiary');
        }
        return colors;
      }
      return ['var(--clo-tertiary)'];
    };

    return {
      chart: {
        height: 300,
        type: 'bar',
        redrawOnWindowResize: true,
        redrawOnParentResize: true,
        zoom: {
          enabled: false,
        },
        fontFamily: "'Lato', Roboto, 'Helvetica Neue', Arial, sans-serif !default",
        toolbar: {
          show: false,
        },
      },
      grid: { borderColor: 'var(--color-light-gray)' },
      plotOptions: {
        bar: {
          distributed: true, // Its is neccesary to display different bar colors
          borderRadius: 0,
          dataLabels: {
            position: 'top',
          },
        },
      },
      legend: {
        show: false, // After enabling 'distributed', we have to hide the legend
      },
      dataLabels: {
        enabled: true,
        offsetY: -20,
        style: {
          fontSize: '12px',
          colors: ['var(--color-font)'],
        },
        formatter: (value: number) => {
          if (value === 0) return '';
          return prettifyNumber(value, 1);
        },
      },
      colors: getBarColors(),
      xaxis: {
        type: 'datetime',
        min: monthlyFormatter ? undefined : moment().subtract(30, 'days').unix() * 1000,
        labels: {
          style: {
            colors: 'var(--color-font)',
            fontSize: '11px',
          },
          format: monthlyFormatter ? 'MM/yy' : undefined,
        },
      },
      yaxis: {
        labels: {
          style: {
            colors: ['var(--color-font)'],
          },
        },
      },
      tooltip: {
        x: {
          formatter: (val: number): string => {
            return monthlyFormatter ? moment(val).format('MM/YY') : moment(val).format('DD MMM YY');
          },
        },
      },
      responsive: [
        {
          breakpoint: 1920,
          options: {
            plotOptions: {
              bar: {
                columnWidth: '80%',
              },
            },
            dataLabels: {
              offsetY: -15,
              style: {
                fontSize: '9px',
              },
            },
          },
        },
        {
          breakpoint: 1400,
          options: {
            dataLabels: {
              offsetY: monthlyFormatter ? -17 : -14,
              style: {
                fontSize: monthlyFormatter ? '10px' : '8px',
              },
            },
          },
        },
        {
          breakpoint: 992,
          options: {
            dataLabels: {
              enabled: false,
            },
          },
        },
        {
          breakpoint: 768,
          options: {
            plotOptions: {
              bar: {
                columnWidth: '50%',
              },
            },
          },
        },
      ],
    };
  };

  const prepareDonutData = (data: { [key: string]: number }[]): number[] => {
    const tmpData: any = {};
    data.forEach((item) => {
      Object.keys(item).forEach((k: string) => {
        tmpData[k] = item[k];
      });
    });
    return Object.values(RatingKind).map((x: string) => {
      if (tmpData.hasOwnProperty(x)) {
        return tmpData[x];
      } else {
        return 0;
      }
    });
  };

  const prepareHeatMapData = (data: DistributionData[]): ApexAxisChartSeries => {
    const series: HeatMapData[] = [];
    const groupedByYear = groupBy(data, 'year');

    // We use 10 by default and add 10 to the rest of values
    // due to a bug displaying proper bg color in heatmap
    Object.keys(groupedByYear).forEach((year: string) => {
      let currentData = new Array(12).fill(10);
      groupedByYear[year].forEach((i: DistributionData) => {
        currentData[i.month - 1] = i.total + 10;
      });
      series.push({ name: year, data: currentData });
    });

    return series;
  };

  async function getStats() {
    try {
      setIsLoading(true);
      const stats = await API.getStats(selectedFoundation);
      setSnapshots(stats.snapshots || []);
      setStats(stats);
      checkCurrentStats(stats);
      setApiError(null);
      setIsLoading(false);
    } catch (err: any) {
      setIsLoading(false);
      setApiError('An error occurred getting CLOMonitor stats.');
      setSnapshots([]);
      setStats(null);
    } finally {
      // Go to hash after getting stats
      scrollIntoView(location.hash);
    }
  }

  useEffect(() => {
    setActiveDate(undefined);
    getStats();
  }, [searchParams]); /* eslint-disable-line react-hooks/exhaustive-deps */

  // Link search page from donut charts
  useEffect(() => {
    if (!isUndefined(selectedPoint)) {
      loadSearchPage(selectedPoint);
    }
  }, [selectedPoint]); /* eslint-disable-line react-hooks/exhaustive-deps */

  // Link search page from heat map
  useEffect(() => {
    if (!isUndefined(selectedRange)) {
      loadSearchPageWithAcceptedRange(selectedRange);
    }
  }, [selectedRange]); /* eslint-disable-line react-hooks/exhaustive-deps */

  useEffect(() => {
    async function fetchSnapshot() {
      setIsLoading(true);
      try {
        const stats = await API.getStatsSnapshot(activeDate!, selectedFoundation);
        setStats(stats);
        checkCurrentStats(stats);
        setIsLoading(false);
      } catch (err: any) {
        setStats(null);
        setIsLoading(false);
      }
    }

    if (stats) {
      if (isUndefined(activeDate)) {
        getStats();
      } else {
        fetchSnapshot();
      }
    }
  }, [activeDate]); /* eslint-disable-line react-hooks/exhaustive-deps */

  const downloadRepositoriesCSV = () => {
    async function getCSV() {
      try {
        setDownloadingCSV(true);
        const csv = await API.getRepositoriesCSV();

        const blob = new Blob([csv], {
          type: 'text/csv',
        });
        const link: HTMLAnchorElement = document.createElement('a');
        link.download = 'repositories.csv';
        link.href = window.URL.createObjectURL(blob);
        link.style.display = 'none';
        document.body.appendChild(link);
        link.click();
        setDownloadingCSV(false);
      } catch {
        alertDispatcher.postAlert({
          type: 'danger',
          message: 'An error occurred downloading the repositories CSV file, please try again later.',
        });
        setDownloadingCSV(false);
      }
    }
    getCSV();
  };

  return (
    <div className="d-flex flex-column flex-grow-1 position-relative">
      <SubNavbar>
        <div className="d-flex flex-column flex-sm-row align-items-center w-100 justify-content-between my-2">
          <div className="d-flex flex-column">
            <div className="h2 text-dark text-center text-md-start">CLOMonitor Stats</div>
            {isNull(apiError) && (
              <small className="d-flex flex-row justify-content-center justify-content-md-start">
                <span className="d-none d-md-block me-2">Report generated at:</span>
                {stats && !isUndefined(stats.generated_at) ? (
                  <span className="fw-bold">{moment(stats.generated_at).format('YYYY/MM/DD HH:mm:ss (Z)')}</span>
                ) : (
                  <div className="d-flex flex-row mt-1" role="status">
                    <div className={`${styles.dot} ${styles.dot1} dot`} />
                    <div className={`${styles.dot} ${styles.dot2} dot`} />
                    <div className={`${styles.dot} ${styles.dot3} dot`} />
                  </div>
                )}
              </small>
            )}
          </div>

          <div className={styles.selectWrapper}>
            <div className="d-flex flex-column ms-0 ms-sm-3 mt-3 mt-sm-0 px-4 px-sm-0">
              <label className="form-label me-2 mb-0 fw-bold">Foundation:</label>
              <select
                className={`form-select rounded-0 cursorPointer foundation ${styles.select}`}
                value={selectedFoundation || ''}
                onChange={handleChange}
                aria-label="Foundation options select"
              >
                <option value="">All</option>
                {Object.keys(FOUNDATIONS).map((f: string) => {
                  const fData = FOUNDATIONS[f as Foundation];
                  if (isUndefined(fData)) return null;
                  return (
                    <option key={`f_${f}`} value={f}>
                      {fData.name}
                    </option>
                  );
                })}
              </select>
            </div>
          </div>
        </div>
      </SubNavbar>
      {isLoading && (
        <Loading
          className={`loadingBg ${styles.loadingWrapper}`}
          spinnerClassName={classNames(styles.loading, { [styles.withContent]: stats })}
        />
      )}
      <main role="main" className="container-lg py-5 position-relative">
        <div className="d-flex flex-row">
          <div
            className={classNames('d-flex flex-column position-relative', styles.statsWrapper, {
              'w-100': !isUndefined(snapshots) && snapshots.length < 3,
            })}
          >
            {apiError && (
              <NoData>
                <div className="mb-4 mb-lg-5 h2">{apiError}</div>
                <p className="h5 mb-0">Please try again later.</p>
              </NoData>
            )}
            {stats && (
              <>
                {emptyStats && (
                  <div>
                    <NoData>
                      <div className="mb-4 mb-lg-5 h2">No Stats available for the moment.</div>
                      <p className="h5 mb-0">Please try again later.</p>
                    </NoData>
                  </div>
                )}

                <>
                  {stats.projects.running_total && stats.projects.accepted_distribution && (
                    <>
                      <div className={`text-dark fw-bold text-uppercase text-center mb-4 ${styles.title}`}>
                        Projects
                      </div>
                      <div className={`text-dark text-center mb-3 fw-bold ${styles.subtitle}`}>Projects accepted</div>

                      <div className="py-4">
                        <div className="row g-4 g-xxl-5 justify-content-center">
                          <div className="col-12 col-md-6 col-xl-8">
                            <div className={`card rounded-0 ${styles.chartWrapper}`}>
                              <div className={`card-body ${styles.reducedPaddingBottom}`}>
                                <ReactApexChart
                                  options={getAreaChartConfig()}
                                  series={[{ name: 'Projects', data: stats.projects.running_total }]}
                                  type="area"
                                  height={250}
                                />
                              </div>
                            </div>
                          </div>

                          <div className="col-12 col-md-6 col-xl-4">
                            <div className={`card rounded-0 ${styles.chartWrapper}`}>
                              <div className={`card-body ${styles.reducedPaddingBottom}`}>
                                <ReactApexChart
                                  options={getHeatMapChartConfig()}
                                  series={prepareHeatMapData(stats.projects.accepted_distribution)}
                                  type="heatmap"
                                  height={250}
                                />
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </>
                  )}

                  {stats.projects.rating_distribution && (
                    <>
                      <div className={`text-dark text-center fw-bold mt-4 mb-3 ${styles.subtitle}`}>
                        Distribution of projects by rating
                      </div>

                      <div className="py-4">
                        <div className="row g-4 g-xxl-5 justify-content-center">
                          <div className="col-6 col-xl-3">
                            <div className={`card rounded-0 ${styles.chartWrapper}`}>
                              <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                                All
                              </div>
                              <div className={`card-body ${styles.donutWrapper}`}>
                                <ReactApexChart
                                  options={getDonutChartConfig()}
                                  series={prepareDonutData(stats.projects.rating_distribution.all)}
                                  type="donut"
                                  height={250}
                                />
                              </div>
                            </div>
                          </div>

                          {stats.projects.rating_distribution.graduated && (
                            <div className="col-6 col-xl-3">
                              <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                                  Graduated
                                </div>
                                <div className={`card-body ${styles.donutWrapper}`}>
                                  <ReactApexChart
                                    options={getDonutChartConfig(Maturity.graduated)}
                                    series={prepareDonutData(stats.projects.rating_distribution.graduated)}
                                    type="donut"
                                    height={250}
                                  />
                                </div>
                              </div>
                            </div>
                          )}

                          {stats.projects.rating_distribution.incubating && (
                            <div className="col-6 col-xl-3">
                              <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                                  Incubating
                                </div>
                                <div className={`card-body ${styles.donutWrapper}`}>
                                  <ReactApexChart
                                    options={getDonutChartConfig(Maturity.incubating)}
                                    series={prepareDonutData(stats.projects.rating_distribution.incubating)}
                                    type="donut"
                                    height={250}
                                  />
                                </div>
                              </div>
                            </div>
                          )}

                          {stats.projects.rating_distribution.sandbox && (
                            <div className="col-6 col-xl-3">
                              <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                                  Sandbox
                                </div>
                                <div className={`card-body ${styles.donutWrapper}`}>
                                  <ReactApexChart
                                    options={getDonutChartConfig(Maturity.sandbox)}
                                    series={prepareDonutData(stats.projects.rating_distribution.sandbox)}
                                    type="donut"
                                    height={250}
                                  />
                                </div>
                              </div>
                            </div>
                          )}
                        </div>
                      </div>
                    </>
                  )}

                  {stats.projects.sections_average && (
                    <>
                      <div className={`text-dark text-center fw-bold mt-4 mb-3 ${styles.subtitle}`}>
                        Projects average score per category
                      </div>

                      <div className="py-4">
                        <div className="row g-4 g-xxl-5 justify-content-center">
                          <div className="col-6 col-xl-3">
                            <div className={`card rounded-0 ${styles.chartWrapper}`}>
                              <Average title="All" data={stats.projects.sections_average.all} />
                            </div>
                          </div>

                          {!isEmpty(stats.projects.sections_average.graduated) && (
                            <div className="col-6 col-xl-3">
                              <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                <Average title="Graduated" data={stats.projects.sections_average.graduated} />
                              </div>
                            </div>
                          )}

                          {!isEmpty(stats.projects.sections_average.incubating) && (
                            <div className="col-6 col-xl-3">
                              <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                <Average title="Incubating" data={stats.projects.sections_average.incubating} />
                              </div>
                            </div>
                          )}

                          {!isEmpty(stats.projects.sections_average.sandbox) && (
                            <div className="col-6 col-xl-3">
                              <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                <Average title="Sandbox" data={stats.projects.sections_average.sandbox} />
                              </div>
                            </div>
                          )}
                        </div>
                      </div>
                    </>
                  )}

                  {stats.repositories.passing_check && (
                    <>
                      <div className={`text-dark text-center fw-bold text-uppercase my-4 ${styles.title}`}>
                        Repositories
                      </div>
                      <div className="d-flex flex-column flex-md-row align-items-baseline justify-content-center mb-3">
                        <div className={`text-dark fw-bold mx-auto mx-md-0 ${styles.subtitle}`}>
                          Percentage of repositories passing each check
                        </div>
                        <button
                          className={`btn btn-link mt-2 mt-md-0 p-0 ps-0 ps-md-3 mx-auto mx-md-0 ${styles.downloadBtn}`}
                          onClick={downloadRepositoriesCSV}
                          aria-label="Download repositories CSV file"
                        >
                          <div className="d-flex flex-row align-items-baseline position-relative">
                            <div>(</div>
                            <div className="me-1">
                              <GrDocumentCsv className={`position-relative ${styles.downloadIcon}`} />
                            </div>
                            <div>Download CSV file)</div>
                            {downloadingCSV && (
                              <div className={`position-absolute ${styles.downloadSpinner}`}>
                                <Loading
                                  spinnerClassName={`position-relative ${styles.miniSpinner}`}
                                  noWrapper
                                  smallSize
                                />
                              </div>
                            )}
                          </div>
                        </button>
                      </div>
                      <div className="py-4">
                        <div className="row no-gutters justify-content-center">
                          <div className="col-12">
                            <div className={`card rounded-0 ${styles.chartWrapper}`}>
                              <div className={`card-body ${styles.checksBody}`}>
                                <div className="row g-4 justify-content-center">
                                  <Checks
                                    title="Documentation"
                                    data={stats.repositories.passing_check.documentation}
                                    onSelectCheck={selectCheck}
                                  />
                                  <Checks
                                    title="License"
                                    data={stats.repositories.passing_check.license}
                                    onSelectCheck={selectCheck}
                                  />
                                  <Checks
                                    title="Best Practices"
                                    data={stats.repositories.passing_check.best_practices}
                                    onSelectCheck={selectCheck}
                                  />
                                  <Checks
                                    title="Security"
                                    data={stats.repositories.passing_check.security}
                                    onSelectCheck={selectCheck}
                                  />
                                  <Checks
                                    title="Legal"
                                    data={stats.repositories.passing_check.legal}
                                    onSelectCheck={selectCheck}
                                  />
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </>
                  )}

                  {(stats.projects.views_daily || stats.projects.views_monthly) && (
                    <>
                      <AnchorHeader
                        title="Usage"
                        className={`text-dark text-center fw-bold text-uppercase my-4 ${styles.title}`}
                        scrollIntoView={scrollIntoView}
                      />

                      {!isUndefined(stats.projects.views_monthly) && (
                        <>
                          <div className={`text-dark text-center mb-3 fw-bold ${styles.subtitle}`}>
                            Projects monthly views
                          </div>

                          <div className="py-4">
                            <div className="row g-4 g-xxl-5 justify-content-center">
                              <div className="col-12">
                                <div className={`card rounded-0 ${styles.chartWrapper}`}>
                                  <div className="card-body">
                                    <ReactApexChart
                                      options={getBarChartConfig(
                                        true,
                                        stats.projects.views_monthly.length,
                                        stats.projects.views_monthly.length > 0
                                          ? stats.projects.views_daily.slice(-1)[0][0]
                                          : undefined
                                      )}
                                      series={[
                                        {
                                          name: 'Monthly views',
                                          data: prepareMonthlyViewsData(stats.projects.views_monthly),
                                        },
                                      ]}
                                      type="bar"
                                      height={250}
                                    />
                                  </div>
                                </div>
                              </div>
                            </div>
                          </div>
                        </>
                      )}

                      <div className={`text-dark text-center my-3 fw-bold ${styles.subtitle}`}>
                        Projects daily views
                      </div>

                      <div className="py-4">
                        <div className="row g-4 g-xxl-5 justify-content-center">
                          <div className="col-12">
                            <div className={`card rounded-0 ${styles.chartWrapper}`}>
                              <div className={`card-body ${styles.reducedPaddingBottom}`}>
                                <ReactApexChart
                                  options={getBarChartConfig(
                                    false,
                                    stats.projects.views_daily.length,
                                    stats.projects.views_daily.length > 0
                                      ? stats.projects.views_daily.slice(-1)[0][0]
                                      : undefined
                                  )}
                                  series={[{ name: 'Daily views', data: stats.projects.views_daily }]}
                                  type="bar"
                                  height={250}
                                />
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </>
                  )}
                </>
              </>
            )}
          </div>

          <Timeline
            snapshots={snapshots || []}
            className={`ms-4 ms-lg-5 ${styles.timeline}`}
            activeDate={activeDate}
            setActiveDate={setActiveDate}
          />
        </div>
      </main>
    </div>
  );
};

export default StatsView;
