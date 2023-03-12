import { defineStore } from "pinia";
import { unref } from "vue";
import { setCssVar } from "quasar";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Card {
  id: number;
  name: string;
  text: string;
  active: boolean;
}

interface State {
  settings: {
    company_view: {
      cards: Card[];
    };
    navigation: {
      next: string | undefined;
      prev: string | undefined;
    };
    theme: {
      logo: string;
      primary: string;
    };
  };
  load_wait: number;
}

export const useSite_settingsStore = defineStore("site_settings", {
  state: (): State => ({
    settings: {
      company_view: {
        cards: [],
      },
      navigation: {
        next: undefined,
        prev: undefined,
      },
      theme: {
        logo: "prepage0.png",
        primary: "#d60000",
      },
    },
    load_wait: 0,
  }),
  actions: {
    getCompanyCards(force = false) {
      return new Promise<void>((resolve, reject) => {
        if (force || this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/settings/company_view")
            .then((resp: any) => {
              this.settings.company_view.cards = resp.data;
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
    setCompanyCards(active_cards: Card[]) {
      return new Promise<void>((resolve) => {
        const all_cards = this.settings.company_view.cards;
        all_cards.forEach((card) => {
          if (active_cards.some((active_card) => active_card.id === card.id)) {
            card.active = true;
          } else {
            card.active = false;
          }
        });

        resolve();
      });
    },
    saveSettings() {
      return new Promise<void>((resolve, reject) => {
        this.axios
          .put("/settings/site", { name: "settings", blob: this.settings })
          .then(() => {
            resolve();
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    fetchSettings(force = false) {
      return new Promise<void>((res, rej) => {
        if (force || this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/settings/site")
            .then((resp: any) => {
              if (resp.data.name === "settings") {
                this.settings = JSON.parse(resp.blob);
                setCssVar("primary", this.settings.theme.primary);
                res(resp);
              }
              rej("Didn't receive settings blob");
            })
            .catch((err: any) => {
              rej(err);
            });
        }
      });
    },
    saveCompanyCards() {
      return new Promise<void>((resolve, reject) => {
        const cards = this.settings.company_view.cards;
        this.axios
          .put("/settings/company_view", cards)
          .then(() => {
            resolve();
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    resetCompanyCards() {
      return new Promise((resolve, reject) => {
        this.axios
          .get("/settings/company_view/reset")
          .then(() => {
            resolve(this.getCompanyCards(true));
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    consumeNext() {
      const temp = unref(this.settings.navigation.next);
      this.settings.navigation.next = undefined;
      return temp;
    },
    consumePrev() {
      const temp = unref(this.settings.navigation.prev);
      this.settings.navigation.prev = undefined;
      return temp;
    },
  },
});
