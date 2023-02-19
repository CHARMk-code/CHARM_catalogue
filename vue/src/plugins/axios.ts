import axios, { type AxiosInstance } from "axios";
import type { PiniaPluginContext } from "pinia";

const config = {
  baseURL: `${import.meta.env.baseURL || import.meta.env.apiUrl || ""}/api`,
  timeout: 60 * 1000, // Timeout
  withCredentials: true, // Check cross-site Access-Control
};

// If we are in a local dev env then change config to match
if (location.hostname == "localhost") {
  config.withCredentials = false;
  //  config.baseURL = "http://localhost:5008/api";
  config.baseURL = "https://catalogue.charm.chalmers.se/api";
}

const axios_instance: AxiosInstance = axios.create(config);

declare module "pinia" {
  export interface PiniaCustomProperties {
    axios: AxiosInstance;
  }
}

export function piniaAxiosPlugin(context: PiniaPluginContext) {
  return { axios: axios_instance };
}

export default axios_instance;
