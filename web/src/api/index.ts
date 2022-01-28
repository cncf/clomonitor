import { isEmpty, isNull, isUndefined } from 'lodash';
import camelCase from 'lodash/camelCase';
import isArray from 'lodash/isArray';
import isObject from 'lodash/isObject';

import { Error, ErrorKind, Project, SearchData, SearchQuery } from '../types';

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

  private toCamelCase(r: any): any {
    if (isArray(r)) {
      return r.map((v) => this.toCamelCase(v));
    } else if (isObject(r)) {
      return Object.keys(r).reduce(
        (result, key) => ({
          ...result,
          [camelCase(key)]: this.toCamelCase((r as any)[key]),
        }),
        {}
      );
    }
    return r;
  }

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
        return this.toCamelCase(json);
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

  public getProjectDetail(pathname: string): Promise<Project> {
    return this.apiFetch({ url: `${this.API_BASE_URL}${pathname}/summary` });
  }

  public searchProjects(query: SearchQuery): Promise<{ items: Project[]; paginationTotalCount: string }> {
    let dataParams: SearchData = { limit: query.limit, offset: query.offset };
    if (query.text) {
      dataParams['text'] = query.text;
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
        // body: JSON.stringify({ limit: 10, offset: 0, category: [], maturity: [], rating: [] }),
        // body: JSON.stringify({ limit: 10, offset: 0, category: [0], maturity: [0, 1], rating: ['a', 'b'] }),
        body: JSON.stringify(dataParams),
      },
    });
  }
}

const API = new API_CLASS();
export default API;
