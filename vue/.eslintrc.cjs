/* eslint-env node */
require("@rushstack/eslint-patch/modern-module-resolution");

module.exports = {
  root: true,
  extends: [
    "plugin:vuetify/base",
    "plugin:vue/vue3-recommended",
    "@vue/eslint-config-typescript",
  ],
};
