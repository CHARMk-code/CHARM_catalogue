import { defineStore } from "pinia"

interface User {
  username?: any,
}


interface State {
  status: string,
  token: string,
  user: User
}

export const useAuthStore = defineStore('auth', {
  state: (): State => ({
    status: "",
    token: localStorage.getItem("token") || "",
    user: {},
  }),
  actions: {
    login(user: any) {
      return new Promise((resolve, reject) => {
        this.status = "loading"
        this.axios
          .post("auth", user)
          .then((resp: any) => {
            this.token = resp.data
            this.status = "success"

            localStorage.setItem("token", this.token);
            this.axios.defaults.headers.common["Authorization"] =
              "basic " + this.token;
            resolve(resp);
          })
          .catch((err: any) => {
            this.token = "error";
            localStorage.removeItem("token");
            reject(err);
          });
      });
    },
    logout() {
      return new Promise<void>((resolve) => {
        this.status = "";
        this.token = "";
        //Should when implemented in backend send the token to the backend for blacklisting
        localStorage.removeItem("token");
        delete this.axios.defaults.headers.common["Authorization"];
        resolve();
      });
    },
  },
  getters: {
    isLoggedIn: (state) => {
      try {
        const expired = JSON.parse(atob(state.token.split(".")[1])).exp;
        const currentTime = Math.floor(Date.now() / 1000);
        return currentTime < expired;
      } catch (e) {
        return false;
      }
    }
  },
});
