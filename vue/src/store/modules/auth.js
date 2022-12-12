import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    status: "",
    token: localStorage.getItem("token") || "",
    user: {},
  }),
  mutations: {
    auth_request(state) {
      state.status = "loading";
    },
    auth_success(state, { token }) {
      state.status = "success";
      state.token = token;
    },
    auth_error(state) {
      state.status = "error";
    },
    logout(state) {
      state.status = "";
      state.token = "";
    },
  },
  actions: {
    login({ commit }, user) {
      return new Promise((resolve, reject) => {
        commit("auth_request");
        Vue.prototype.$axios
          .post("auth", user)
          .then((resp) => {
            const token = resp.data;

            localStorage.setItem("token", token);
            Vue.prototype.$axios.defaults.headers.common["Authorization"] =
              "basic " + token;
            commit("auth_success", { token });
            resolve(resp);
          })
          .catch((err) => {
            commit("auth_error");
            localStorage.removeItem("token");
            reject(err);
          });
      });
    },
    logout({ commit }) {
      return new Promise((resolve) => {
        commit("logout");
        //Should when implemented in backend send the token to the backend for blacklisting
        localStorage.removeItem("token");
        delete Vue.prototype.$axios.defaults.headers.common["Authorization"];
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
    },
    authStatus: (state) => state.status,
    token: (state) => state.token,
  },
};
