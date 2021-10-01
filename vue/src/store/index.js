import Vue from "vue";
import Vuex from "vuex";
import Axios from "@/plugins/axios";
import { HTTP } from "@/plugins/http-common.js";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    auth: {
      status: "",
      token: localStorage.getItem("token") || "",
      user: {},
    },
    companies: [],
    tags: [],
    companyTags: {},
  },
  mutations: {
    auth_request(state) {
      state.auth.status = "loading";
    },
    auth_success(state, { token, user }) {
      state.auth.status = "success";
      state.auth.token = token;
      state.auth.user = user;
    },
    auth_error(state) {
      state.auth.status = "error";
    },
    logout(state) {
      state.auth.status = "";
      state.auth.token = "";
    },
    getData(state, { target, data }) {
      state[target] = data;
    },
    addCompany: (state, company) => {
      state.companies.push(company);
    },
    setCompanies: (state, companies) => {
      state.companies = companies;
    },
    addTag: (state, tag) => {
      state.tags.push(tag);
    },
    setTags: (state, tags) => {
      state.tags = tags;
    },
    addTagstoCompany: (state, { companyId, tags }) => {
      state.companyTags[companyId] = tags;
    },
  },
  actions: {
    login({ commit }, user) {
      return new Promise((resolve, reject) => {
        commit("auth_request");
        Axios({ url: "/auth/login", data: user, method: "POST" })
          .then((resp) => {
            console.log(resp);
            const token = resp.data.token;
            const user = resp.data.user;
            localStorage.setItem("token", token);
            Axios.defaults.headers.common["Authorization"] = "basic " + token;
            commit("auth_success", { token, user });
            resolve(resp);
          })
          .catch((err) => {
            console.log(err);
            commit("auth_error");
            localStorage.removeItem("token");
            reject(err);
          });
      });
    },
    logout({ commit }) {
      return new Promise((resolve) => {
        commit("logout");
        localStorage.removeItem("token");
        delete Axios.defaults.headers.common["Authorization"];
        resolve();
      });
    },
    /* TODO: ADD generic action for loading stuff from backend */
    getData({ commit }, target) {
      return new Promise((resolve) => {
        Axios({
          url: "/" + target + "/get",
          headers: {
            Authorization: "Bearer " + localStorage.getItem("token"),
          },
          method: "GET",
        }).then((resp) => {
          console.log(resp);
          const data = resp["data"];
          commit("getData", { target, data });
          resolve(resp);
        });
      });
    },
    retrieveCompanies({ commit }) {
      return new Promise((resolve) => {
        HTTP.get("company/get").then((resp) => {
          commit("setCompanies", resp.data);
        });
        resolve();
      });
    },
    retrieveTags({ commit }) {
      return new Promise((resolve) => {
        HTTP.get("tag/get").then((resp) => {
          commit("setTags", resp.data);
        });
        resolve();
      });
    },
    retrieveTagsForCompany: ({ commit }, companyId) => {
      return new Promise((resolve) => {
        HTTP.get("tag/get?company_filter=" + companyId).then((resp) => {
          commit("addTagstoCompany", { companyId, tags: resp.data });
        });
        resolve();
      });
    },
  },
  getters: {
    isLoggedIn: (state) => !!state.auth.token,
    authStatus: (state) => state.auth.status,
    /* TODO: Add get to access objects */
    allData: (state) => (target) => {
      if (!Object.keys(state).includes(target)) {
        state.dispatch("getData", target);
      }
      console.log(state[target]);
      return state[target];
    },
    getCompanyById: (state) => (id) => {
      return state.companies.find((company) => company.id === parseFloat(id));
    },
    getTagById: (state) => (id) => {
      return state.tags.find((tag) => tag.id === parseFloat(id));
    },
    getTagsByCompanyId: (state) => (id) => {
      return state.tags[id];
    },
  },
  // plugins: [
  //   searchPlugin({
  //     resources: {
  //       companies: {
  //         index: ['name'],
  //         getter: state => state.companies,
  //         watch: { delay: 500 }
  //       },
  //       tags: {
  //         index: ['name'],
  //         getter: state => state.tags,
  //         watch: { delay: 500 }
  //       }
  //     }
  //   })
  // ]
});
