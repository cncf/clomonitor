import { isEmpty, isNull, isUndefined } from 'lodash';
import isArray from 'lodash/isArray';

import { DEFAULT_SORT_BY, DEFAULT_SORT_DIRECTION } from '../data';
import { Error, ErrorKind, Project, ProjectDetail, SearchData, SearchQuery, Stats } from '../types';

interface FetchOptions {
  method: 'POST' | 'GET' | 'PUT' | 'DELETE' | 'HEAD';
  headers?: {
    [key: string]: string;
  };
  body?: any;
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

  private getHeadersValue(res: any, params?: string[]): any {
    if (!isUndefined(params) && params.length > 0) {
      let headers: any = {};
      params.forEach((param: string) => {
        if (res.headers.has(param)) {
          headers[param] = res.headers.get(param);
        }
      });
      return headers;
    }
    return null;
  }

  private async processFetchOptions(opts?: FetchOptions): Promise<FetchOptions | any> {
    let options: FetchOptions | any = opts || {};
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

  private async handleErrors(res: any) {
    if (!res.ok) {
      let error: Error;
      switch (res.status) {
        default:
          try {
            let text = await res.json();
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

  private async handleContent(res: any, headers?: string[]) {
    let response = res;

    switch (response.headers.get('Content-Type')) {
      case 'text/plain; charset=utf-8':
      case 'text/markdown':
      case 'application/yaml':
        const text = await response.text();
        return text;
      case 'application/json':
        let json = await response.json();
        const tmpHeaders = this.getHeadersValue(res, headers);
        if (!isNull(tmpHeaders)) {
          if (isArray(json)) {
            json = { items: json };
          }
          json = { ...json, ...tmpHeaders };
        }
        return json;
      default:
        return response;
    }
  }

  private async apiFetch(props: APIFetchProps) {
    let options: FetchOptions | any = await this.processFetchOptions(props.opts);

    return fetch(props.url, options)
      .then(this.handleErrors)
      .then((res) => this.handleContent(res, props.headers))
      .catch((error) => Promise.reject(error));
  }

  public getProjectDetail(org: string, project: string, foundation: string): Promise<ProjectDetail> {
    return this.apiFetch({ url: `${this.API_BASE_URL}/projects/${foundation}/${org}/${project}` });
  }

  public getStats(foundation: string | null): Promise<Stats> {
    return this.apiFetch({
      url: `${this.API_BASE_URL}/stats${!isNull(foundation) ? `?foundation=${foundation}` : ''}`,
    });
  }

  public searchProjects(query: SearchQuery): Promise<{ items: Project[]; 'Pagination-Total-Count': string }> {
    let dataParams: SearchData = {
      limit: query.limit,
      offset: query.offset,
      sort_by: query.sort_by || DEFAULT_SORT_BY,
      sort_direction: query.sort_direction || DEFAULT_SORT_DIRECTION,
    };
    if (query.text) {
      dataParams['text'] = query.text;
    }
    if (query.accepted_from) {
      dataParams['accepted_from'] = query.accepted_from;
    }
    if (query.accepted_to) {
      dataParams['accepted_to'] = query.accepted_to;
    }
    if (!isEmpty(query.filters)) {
      dataParams = { ...dataParams, ...query.filters };
    }
    return this.apiFetch({
      url: `${this.API_BASE_URL}/projects/search`,
      headers: [this.HEADERS.pagination],
      opts: {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(dataParams),
      },
    });
  }
}

const API = new API_CLASS();
export default API;
