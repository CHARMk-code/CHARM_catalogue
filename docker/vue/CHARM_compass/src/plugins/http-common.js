import axios from 'axios'

export const HTTP = axios.create({
  baseURL: `https://compass.glimfjord.com/api/`
})
