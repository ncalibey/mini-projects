import axios, { AxiosPromise } from 'axios';

// HasId ensures that a given implementation has an `id` property.
interface HasId {
  id?: number;
}

// ApiSync syncs the data between a model and a HTTP backend.
export class ApiSync<T extends HasId> {
  constructor(public rootUrl: string) {}

  fetch(id: number): AxiosPromise {
    return axios.get(`${this.rootUrl}/${id}`);
  }

  save(data: T): AxiosPromise {
    const { id } = data;

    if (id) {
      return axios.put(`${this.rootUrl}/${id}`, data);
    } else {
      return axios.post(this.rootUrl, data);
    }
  }
}
