import { defineStore } from "pinia";
import { unref } from "vue";
import { setCssVar } from "quasar";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Card {
  name: string;
  text: string;
  active: boolean;
}

export interface State {
  server_settings: {
    company_view: {
      cards: Card[];
    };
    theme: {
      logo: string;
      primary: string;
    };
  };
  session_settings: {
    tables: {
      rowsPerPage: number;
    };
    navigation: {
      next: string | undefined;
      prev: string | undefined;
    };
  };
  load_wait: number;
}

export const useSite_settingsStore = defineStore("site_settings", {
  state: (): State => ({
    server_settings: {
      company_view: {
        cards: [
          { text: "Logo", name: "logo", active: true },
          { text: "Name", name: "name", active: true },
          { text: "Description", name: "desc", active: true },
          { text: "Did you know", name: "didyouknow", active: true },
          { text: "Divisions", name: "tag_divisions", active: true },
          { text: "Business Areas", name: "tag_business_areas", active: true },
          { text: "Offering", name: "tag_offering", active: true },
          { text: "Looking for", name: "tag_looking_for", active: true },
          { text: "Website", name: "website", active: true },
          { text: "Map", name: "map", active: true },
          { text: "Notes", name: "notes", active: true },
          { text: "CHARMtalks", name: "CHARMtalks", active: true },
          { text: "Language", name: "language", active: true },
          { text: "Fair Areas", name: "fair_area", active: true },
        ],
      },
      theme: {
        logo: "logo.png",
        primary: "#d6d600",
      },
    },
    session_settings: {
      tables: {
        rowsPerPage: 20,
      },
      navigation: {
        next: undefined,
        prev: undefined,
      },
    },
    load_wait: 0,
  }),
  actions: {
    saveSettings() {
      console.log("test");
      return new Promise<void>((resolve, reject) => {
        this.axios
          .put("/v2/settings/blob/", { name: "settings", blob: this.server_settings })
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
            .get("/v2/settings/blob/settings")
            .then((resp: any) => {
              if (resp.data.name === "settings") {
                this.server_settings = resp.data.blob; 
                setCssVar("primary", this.server_settings.theme.primary);
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
    setCompanyCards(active_cards: Card[]) {
      return new Promise<void>((resolve) => {
        const all_cards = this.server_settings.company_view.cards;
        all_cards.forEach((card) => {
          if (
            active_cards.some((active_card) => active_card.name === card.name)
          ) {
            card.active = true;
          } else {
            card.active = false;
          }
        });

        resolve();
      });
    },
    consumeNext() {
      const temp = unref(this.session_settings.navigation.next);
      this.session_settings.navigation.next = undefined;
      return temp;
    },
    consumePrev() {
      const temp = unref(this.session_settings.navigation.prev);
      this.session_settings.navigation.prev = undefined;
      return temp;
    },
    getTablePagination() {
      return {
        sortBy: "desc",
        descending: false,
        page: 1,
        rowsPerPage: this.session_settings.tables.rowsPerPage,
      };
    },
  },
});
