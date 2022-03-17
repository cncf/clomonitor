import { groupBy, isNull, isUndefined } from 'lodash';
import moment from 'moment';
import { useContext, useEffect, useState } from 'react';
import ReactApexChart from 'react-apexcharts';

import API from '../../api';
import { AppContext } from '../../context/AppContextProvider';
import { DistributionData, RatingKind, Stats } from '../../types';
import Loading from '../common/Loading';
import NoData from '../common/NoData';
import SubNavbar from '../navigation/SubNavbar';
import Average from './Average';
import Checks from './Checks';
import styles from './StatsView.module.css';

interface Props {
  hash?: string;
}

interface HeatMapData {
  name: string;
  data: number[];
}

const StatsView = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const [isLightActive, setIsLightActive] = useState<boolean>(effective === 'light');
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [emptyStats, setEmptyStats] = useState<boolean>(false);
  const [stats, setStats] = useState<Stats | null | undefined>();
  const [apiError, setApiError] = useState<string | null>(null);

  useEffect(() => {
    setIsLightActive(effective === 'light');
  }, [effective]);

  const checkCurrentStats = (currentStats: Stats | null) => {
    if (!isNull(currentStats)) {
      const notEmptyItems = Object.keys(currentStats).some((elem: string) => {
        return elem !== 'generated_at' && (currentStats as any)[elem].total !== 0;
      });
      setEmptyStats(!notEmptyItems);
    }
  };

  const getAreaChartConfig = (): ApexCharts.ApexOptions => {
    return {
      chart: {
        fontFamily: "'Lato', Roboto, 'Helvetica Neue', Arial, sans-serif !default",
        height: 250,
        type: 'area',
        redrawOnParentResize: false,
        zoom: {
          type: 'x',
          enabled: true,
          autoScaleYaxis: true,
          zoomedArea: {
            fill: {
              color: 'var(--rm-secondary-15)',
              opacity: 0.4,
            },
            stroke: {
              color: 'var(--rm-secondary-900)',
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

  const getDonutChartConfig = (): ApexCharts.ApexOptions => {
    return {
      chart: {
        fontFamily: "'Lato', Roboto, 'Helvetica Neue', Arial, sans-serif !default",
        height: 250,
        type: 'donut',
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
        toolbar: {
          show: false,
        },
      },
      theme: {
        mode: isLightActive ? 'light' : 'dark',
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
      plotOptions: {
        heatmap: {
          radius: 0,
          shadeIntensity: 0,
          colorScale: {
            inverse: false,
            ranges: [
              {
                from: 0,
                to: 0,
                color: '#cccccc',
              },
              {
                from: 1,
                to: 1,
                color: '#d2e5c4',
              },
              {
                from: 2,
                to: 4,
                color: '#90be6d',
              },
              {
                from: 5,
                to: 100,
                color: '#567241',
              },
            ],
          },
        },
      },
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

    Object.keys(groupedByYear).forEach((year: string) => {
      let currentData = new Array(12).fill(0);
      groupedByYear[year].forEach((i: DistributionData) => {
        currentData[i.month - 1] = i.total;
      });
      series.push({ name: year, data: currentData });
    });

    return series;
  };

  useEffect(() => {
    async function getStats() {
      try {
        setIsLoading(true);
        const stats = await API.getStats();
        setStats(stats);
        checkCurrentStats(stats);
        setApiError(null);
        setIsLoading(false);
      } catch (err: any) {
        setIsLoading(false);
        setApiError('An error occurred getting CLOMonitor stats, please try again later.');
        setStats(null);
      }
    }
    getStats();
  }, []); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <div className="d-flex flex-column flex-grow-1 position-relative">
      <SubNavbar>
        <div className="d-flex flex-column align-items-center justify-content-center w-100 my-2">
          <div className="h2 text-secondary">CLOMonitor Stats</div>
          {stats && (
            <small>
              <span className="me-2">Report generated at:</span>
              {!isUndefined(stats.generated_at) ? (
                <span className="fw-bold">{moment(stats.generated_at).format('YYYY/MM/DD HH:mm:ss (Z)')}</span>
              ) : (
                <div className="d-inline text-primary" role="status">
                  <span className="spinner-border spinner-border-sm" />
                </div>
              )}
            </small>
          )}
        </div>
      </SubNavbar>
      <main role="main" className="container-lg px-sm-4 px-lg-0 py-5">
        <div className="flex-grow-1 position-relative">
          {apiError && <NoData>{apiError}</NoData>}
          {isLoading && <Loading />}
          {stats && (
            <>
              {emptyStats && (
                <div>
                  <NoData>No Stats available for the moment</NoData>
                </div>
              )}

              {stats.projects.running_total && stats.projects.accepted_distribution && (
                <>
                  <div className={`text-dark fw-bold text-center text-uppercase mb-4 ${styles.title}`}>Projects</div>
                  <div className="text-dark text-center mb-3 fw-bold">Projects accepted by the CNCF</div>

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
                  <div className="text-dark fw-bold text-center mt-4 mb-3">Distribution of projects by rating</div>

                  <div className="py-4">
                    <div className="row g-4 g-xxl-5 justify-content-center">
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                            All
                          </div>
                          <div className="card-body">
                            <ReactApexChart
                              options={getDonutChartConfig()}
                              series={prepareDonutData(stats.projects.rating_distribution.all)}
                              type="donut"
                              height={250}
                            />
                          </div>
                        </div>
                      </div>
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                            Graduated
                          </div>
                          <div className="card-body">
                            <ReactApexChart
                              options={getDonutChartConfig()}
                              series={prepareDonutData(stats.projects.rating_distribution.graduated)}
                              type="donut"
                              height={250}
                            />
                          </div>
                        </div>
                      </div>
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                            Incubating
                          </div>
                          <div className="card-body">
                            <ReactApexChart
                              options={getDonutChartConfig()}
                              series={prepareDonutData(stats.projects.rating_distribution.incubating)}
                              type="donut"
                              height={250}
                            />
                          </div>
                        </div>
                      </div>
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>
                            Sandbox
                          </div>
                          <div className="card-body">
                            <ReactApexChart
                              options={getDonutChartConfig()}
                              series={prepareDonutData(stats.projects.rating_distribution.sandbox)}
                              type="donut"
                              height={250}
                            />
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </>
              )}

              {stats.projects.sections_average && (
                <>
                  <div className="text-dark fw-bold text-center mt-4 mb-3">Projects average score per category</div>

                  <div className="py-4">
                    <div className="row g-4 g-xxl-5 justify-content-center">
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <Average title="All" data={stats.projects.sections_average.all} />
                        </div>
                      </div>
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <Average title="Graduated" data={stats.projects.sections_average.graduated} />
                        </div>
                      </div>
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <Average title="Incubating" data={stats.projects.sections_average.incubating} />
                        </div>
                      </div>
                      <div className="col-6 col-xl-3">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <Average title="Sandbox" data={stats.projects.sections_average.sandbox} />
                        </div>
                      </div>
                    </div>
                  </div>
                </>
              )}

              {stats.repositories.passing_check && (
                <>
                  <div className={`text-dark fw-bold text-center text-uppercase my-4 ${styles.title}`}>
                    Repositories
                  </div>
                  <div className="text-dark fw-bold text-center mb-3">
                    Percentage of repositories passing each check
                  </div>
                  <div className="py-4">
                    <div className="row no-gutters justify-content-center">
                      <div className="col-12">
                        <div className={`card rounded-0 ${styles.chartWrapper}`}>
                          <div className="card-body p-4">
                            <div className="row g-4 justify-content-center">
                              <Checks title="Documentation" data={stats.repositories.passing_check.documentation} />
                              <Checks title="License" data={stats.repositories.passing_check.license} />
                              <Checks title="Best Practices" data={stats.repositories.passing_check.best_practices} />
                              <Checks title="Security" data={stats.repositories.passing_check.security} />
                              <Checks title="Legal" data={stats.repositories.passing_check.legal} />
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </>
              )}
            </>
          )}
        </div>
      </main>
    </div>
  );
};

export default StatsView;
