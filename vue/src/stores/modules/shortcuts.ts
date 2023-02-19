import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Shortcut {
  id?: number,
  name: string,
  desc: string,
  link: string,
  icon: string,
}

/*
shortcutsStore.modifyShortcut({"id": 1, "name": "test", "desc": "testarmycket", "link": "https://coollink.se", "icon": "mdi-search"})
*/


interface State {
  shortcuts: Shortcut[],
  load_wait: number,
}

export const useShortcutsStore = defineStore('shortcuts', {
  state: (): State => ({
    shortcuts: [],
    load_wait: 0,
  }),
  actions: {
    getShortcuts() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/shortcut")
            .then((resp: any) => {
              this.shortcuts = []
              const shortcuts = resp.data;
              if (shortcuts.length > 0) {
                this.shortcuts = shortcuts
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
    modifyShortcut(shortcut: Shortcut) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/shortcut", shortcut)
          .then((resp: any) => {
            if (!shortcut.id) {
              shortcut.id = resp.data.id;
            }
            if (!this.shortcuts.some((p) => p.id === shortcut.id)) {
              this.shortcuts.push(shortcut);
            } else {
              this.shortcuts[this.shortcuts.findIndex((p) => p.id === shortcut.id)];
            }
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    deleteShortcut(shortcut: Shortcut) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/shortcut/" + shortcut.id)
          .then((resp: any) => {
            this.shortcuts = this.shortcuts.filter((p) => p.id != shortcut.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
});
