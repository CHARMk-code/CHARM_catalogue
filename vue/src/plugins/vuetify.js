import Vue from "vue";
import Vuetify from "vuetify";
// import "vuetify/dist/vuetify.min.css";

Vue.use(Vuetify);

const opts = {
  theme: {
    options: {
      customProperties: true,
    },
    dark: false,
    themes: {
      light: {
        primary: "#d60000",
        secondary: "#9c0000",
      },
    },
  },
};

export default new Vuetify(opts);
