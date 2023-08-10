import { defineStore } from "pinia";
import { unref } from "vue";

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
    tables: {
      rowsPerPage: number;
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
      tables: {
        rowsPerPage: 10,
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
            .get("/v2/settings/company_view/")
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
    saveCompanyCards() {
      return new Promise<void>((resolve, reject) => {
        const cards = this.settings.company_view.cards;
        this.axios
          .put("/v2/settings/company_view/", cards)
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
          .get("/v2/settings/company_view/reset/")
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
    getTablePagination() {
      return {
        sortBy: "desc",
        descending: false,
        page: 1,
        rowsPerPage: this.settings.tables.rowsPerPage,
      };
    },
  },
});
