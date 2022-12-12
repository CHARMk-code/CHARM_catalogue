import { defineStore } from "pinia"
import dayjs from "dayjs"

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Company {
  id: number,
  last_updated: Date,
  active: boolean,
  charmtalk: boolean,
  name: string,
  description: string,
  unique_selling_point: string,
  summer_job_description: string,
  summer_job_link: string,
  summer_job_deadline: string, //Should maybe be date
  contacts: string,
  contact_email: string,
  employees_world: number,
  employees_sweden: number,
  website: string,
  talk_to_us_about: string,
  logo: string,
  map_image: string,
  booth_number: number,
  tags: number[],
  looking_for?: number[],
  business_areas?: number[],
  offerings?: number[],
  divisions?: number[],
}


interface State {
  companies: Company[],
  load_wait: number
}

export const useCompaniesStore = defineStore('companies', {
  state: (): State => ({
    companies: [],
    load_wait: 0,
  }),
  actions: {
    setAllCompanies(companies: Company[]) {
      this.companies = companies;
    },
    removeCompany(id: number) {
      this.companies = this.companies.filter((c) => c.id != id);
    },
    removeAllCompanies() {
      this.companies = [];
    },
    getCompanies() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/company")
            .then((resp: any) => {
              this.removeAllCompanies()
              const companies: Company[] = resp.data;

              // Work around to get summer job deadline in correct format
              companies.forEach(comp => {
                comp.summer_job_deadline =
                  dayjs(comp.summer_job_deadline, "DD MMM YYYY HH:mm:ss", false).format("YYYY-MM-DD");
              });

              if (companies.length > 0) {
                this.setAllCompanies(companies)
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
    modifyCompany(company: Company) {
      return new Promise((resolve, reject) => {
        const updated_company = {
          ...company,
          tags: company.looking_for.map((o) => o.id) //This will be changed when the parts relying on this has changed
            .concat(company.offering.map((o) => o.id))
            .concat(company.divisions.map((o) => o.id))
            .concat(company.business_area.map((o) => o.id)),
        };
        this.axios
          .put("/company", company)
          .then((resp: any) => {
            if (!this.companies.some((c) => (c.id = company.id))) {
              this.companies.push(company);
            } else {
              this.companies[this.companies.findIndex((c) => c.id == company.id)];
            }
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    deleteCompany(company: Company) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/company/" + company.id)
          .then((resp: any) => {
            this.removeCompany(company.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    companyByName: (state) => (name: string) => {
      return state.companies.filter((c) => c.name == name);
    },
  },
});
