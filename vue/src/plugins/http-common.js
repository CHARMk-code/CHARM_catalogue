import axios from "axios";

export const HTTP = axios.create({
  baseURL: `https://catalogue.glimfjord.com/api/`,
});
