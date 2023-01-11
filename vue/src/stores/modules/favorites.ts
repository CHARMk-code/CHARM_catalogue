import { defineStore } from "pinia"

interface State {
  favorites: Set<number>,
}

export const useFavoritesStore = defineStore('favorites', {
  state: (): State => ({
    favorites: new Set(),
  }),
  actions: {
    addFavorite(id: number) {
      this.favorites.add(id);

      localStorage.setItem(
        "favorites",
        JSON.stringify(Array.from(this.favorites.values()))
      );
    },
    removeFavorite(id: number) {
      this.favorites.delete(id);

      localStorage.setItem(
        "favorites",
        JSON.stringify(Array.from(this.favorites.values()))
      );
    },
    toggleFavorite(id: number) {
      if (!this.isFavorite(id)) {
        this.addFavorite(id)
      } else {
        this.removeFavorite(id)
      }
    },
    loadFromStorage() {
      const stored_favorites = localStorage.getItem("favorites") || "[]"
      if (stored_favorites == "{}") {
        this.favorites = new Set();
      } else {
        this.favorites = new Set(JSON.parse(stored_favorites));
      }
    },
  },
  getters: {
    isFavorite: (state) => (id: number) => {
      return state.favorites.has(id);
    },
  },
});