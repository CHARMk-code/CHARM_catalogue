module.exports = {
  runtimeCompiler: true,
  transpileDependencies: ["vuetify"],
  configureWebpack: {
    devServer: {
      disableHostCheck: true,
    },
  },
};
