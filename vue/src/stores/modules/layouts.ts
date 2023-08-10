import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Layout {
  id: number;
  image: string;
  active: boolean;
  placement: number;
}

interface State {
  layouts: Layout[];
  load_wait: number;
}

export const useLayoutsStore = defineStore("layouts", {
  state: (): State => ({
    layouts: [],
    load_wait: 0,
  }),
  actions: {
    setAllLayouts(layouts: Layout[]) {
      this.layouts = layouts;
    },
    removeLayout(id: number) {
      this.layouts = this.layouts.filter((p) => p.id != id);
    },
    removeAllLayouts() {
      this.layouts = [];
    },
    getLayouts() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/v2/layout/")
            .then((resp: any) => {
              this.removeAllLayouts();
              const layouts = resp.data;
              if (layouts.length > 0) {
                this.setAllLayouts(layouts);
              }
              resolve(resp);
            })
            .catch((err: any) => {
              reject(err);
            });
        } else {
          resolve();
        }
      });
    },
    modifyLayout(layout: Layout) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/v2/layout/", layout)
          .then((resp: any) => {
            if (!this.layouts.some((p) => (p.id = layout.id))) {
              this.layouts.push(layout);
            } else {
              this.layouts[this.layouts.findIndex((p) => p.id == layout.id)];
            }
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    deleteLayout(layout: Layout) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/v2/layout/" + layout.id)
          .then((resp: any) => {
            this.removeLayout(layout.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getSide: (state) => (side: "left" | "right") => {
      const side_number = side === "left" ? 1 : 2;
      const layout = state.layouts.filter(
        (t) => t.placement === side_number && t.active
      );
      return layout.length === 0 ? undefined : layout[0];
    },
    getCenter: (state) => {
      const layouts = state.layouts.filter((t) => t.placement === 0);
      if (layouts.length !== 0) {
        const index = Math.floor(Math.random() * layouts.length);
        return layouts[index];
      }
    },
  },
});
