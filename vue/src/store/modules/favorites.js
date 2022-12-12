export default {
  namespaced: true,
  state: () => ({
    favorites: new Set(),
  }),
  mutations: {
    addFavorite(state, id) {
      state.favorites.add(id);

      localStorage.setItem(
        "favorites",
        JSON.stringify(Array.from(state.favorites.values()))
      );
    },
    removeFavorite(state, id) {
      state.favorites.delete(id);

      localStorage.setItem(
        "favorites",
        JSON.stringify(Array.from(state.favorites.values()))
      );
    },
    loadForStorage(state) {
      if (
        localStorage.getItem("favorites") == "{}" ||
        localStorage.getItem("favorites") == null
      ) {
        state.favorites = new Set();
      } else {
        state.favorites = new Set(
          JSON.parse(localStorage.getItem("favorites"))
        );
      }
    },
  },
  getters: {
    favorites: (state) => {
      return state.favorites;
    },
    isInFavorites: (state, id) => {
      return state.favorites.has(id);
    },
  },
};
