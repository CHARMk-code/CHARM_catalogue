import axios from "axios";

let config = {
  baseURL: `${import.meta.env.baseURL || import.meta.env.apiUrl || ""}/api`,
  timeout: 60 * 1000, // Timeout
  withCredentials: true, // Check cross-site Access-Control
};

// If we are in a local dev env then change config to match
if (location.hostname == "localhost") {
  config.withCredentials =  false;
  config.baseURL = "http://localhost:5008/api";
}

const _axios = axios.create(config);

export default _axios