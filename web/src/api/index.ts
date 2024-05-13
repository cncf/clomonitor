import { isEmpty, isNull, isUndefined } from 'lodash';
import isArray from 'lodash/isArray';

import { DEFAULT_SORT_BY, DEFAULT_SORT_DIRECTION } from '../data';
import { Error, ErrorKind, Project, ProjectDetail, SearchQuery, Stats } from '../types';

interface FetchOptions {
  method: 'POST' | 'GET' | 'PUT' | 'DELETE' | 'HEAD';
  headers?: {
    [key: string]: string;
  };
  body?: string;
}

interface APIFetchProps {
  url: string;
  opts?: FetchOptions;
  headers?: string[];
}

class API_CLASS {
  private API_BASE_URL = '/api';
  private HEADERS = {
    pagination: 'Pagination-Total-Count',
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private getHeadersValue(res: any, params?: string[]): { [key: string]: string } | null {
    if (!isUndefined(params) && params.length > 0) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const headers: any = {};
      params.forEach((param: string) => {
        if (res.headers.has(param)) {
          headers[param] = res.headers.get(param);
        }
      });
      return headers;
    }
    return null;
  }

  private async processFetchOptions(opts?: FetchOptions): Promise<FetchOptions | object> {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const options: FetchOptions | any = opts || {};
    if (opts && ['DELETE', 'POST', 'PUT'].includes(opts.method)) {
      return {
        ...options,
        headers: {
          ...options.headers,
        },
      };
    }
    return options;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private async handleErrors(res: any) {
    if (!res.ok) {
      let error: Error;
      switch (res.status) {
        default:
          try {
            const text = await res.json();
            error = {
              kind: ErrorKind.Other,
              message: text.message !== '' ? text.message : undefined,
            };
          } catch {
            error = {
              kind: ErrorKind.Other,
            };
          }
      }
      throw error;
    }
    return res;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private async handleContent(res: any, headers?: string[]) {
    const response = res;
    let content;
    let tmpHeaders;

    switch (response.headers.get('Content-Type')) {
      case 'text/plain; charset=utf-8':
      case 'text/markdown':
      case 'csv':
        content = await response.text();
        return content;
      case 'application/json':
        content = await response.json();
        tmpHeaders = this.getHeadersValue(res, headers);
        if (!isNull(tmpHeaders)) {
          if (isArray(content)) {
            content = { items: content };
          }
          content = { ...content, ...tmpHeaders };
        }
        return content;
      default:
        return response;
    }
  }

  private async apiFetch(props: APIFetchProps) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const options: FetchOptions | any = await this.processFetchOptions(props.opts);

    return fetch(props.url, options)
      .then(this.handleErrors)
      .then((res) => this.handleContent(res, props.headers))
      .catch((error) => Promise.reject(error));
  }

  public getProjectDetail(project: string, foundation: string): Promise<ProjectDetail> {
    return this.apiFetch({ url: `${this.API_BASE_URL}/projects/${foundation}/${project}` });
  }

  public getStats(foundation: string | null): Promise<Stats> {
    return this.apiFetch({
      url: `${this.API_BASE_URL}/stats${!isNull(foundation) ? `?foundation=${foundation}` : ''}`,
    });
  }

  public searchProjects(query: SearchQuery): Promise<{ items: Project[]; 'Pagination-Total-Count': string }> {
    let q: string = `limit=${query.limit}&offset=${query.offset}&sort_by=${
      query.sort_by || DEFAULT_SORT_BY
    }&sort_direction=${query.sort_direction || DEFAULT_SORT_DIRECTION}`;

    if (query.text) {
      q += `&text=${query.text}`;
    }
    if (query.accepted_from) {
      q += `&accepted_from=${query.accepted_from}`;
    }
    if (query.accepted_to) {
      q += `&accepted_to=${query.accepted_to}`;
    }
    if (!isUndefined(query.filters) && !isEmpty(query.filters)) {
      Object.keys(query.filters!).forEach((k: string) => {
        query.filters![k].forEach((f: string, index: number) => {
          q += `&${k}[${index}]=${f}`;
        });
      });
    }
    return this.apiFetch({
      url: `${this.API_BASE_URL}/projects/search?${q}`,
      headers: [this.HEADERS.pagination],
      opts: {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      },
    });
  }

  public getRepositoriesCSV(): Promise<string> {
    return this.apiFetch({
      url: '/data/repositories.csv',
    });
  }

  public getProjectSnapshot(project: string, foundation: string, date: string): Promise<ProjectDetail> {
    return this.apiFetch({ url: `${this.API_BASE_URL}/projects/${foundation}/${project}/snapshots/${date}` });
  }

  public getStatsSnapshot(date: string, foundation: string | null): Promise<Stats> {
    return this.apiFetch({
      url: `${this.API_BASE_URL}/stats/snapshots/${date}${!isNull(foundation) ? `?foundation=${foundation}` : ''}`,
    });
  }

  public getRepositoryReportMD(foundation: string, project: string, repoName: string): Promise<string> {
    return this.apiFetch({
      url: `${this.API_BASE_URL}/projects/${foundation}/${project}/${repoName}/report.md`,
    });
  }

  public trackView(projectId: string): Promise<null> {
    return this.apiFetch({
      url: `${this.API_BASE_URL}/projects/views/${projectId}`,
      opts: {
        method: 'POST',
      },
    });
  }
}

const API = new API_CLASS();
export default API;
