import type { TextJustify } from "ol/style/Text";
import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface FairMap {
  id: number;
  name: string;
  background: string;
  styling: FairMapStyle;
  mapGeometry: MapGeometry[];
}

export interface FairMapStyle {
  circleSize?: number;
  maxZoom?: number;
  mapSize: [number, number];
}

export type GeometryTypes = "company" | "other";

export interface MapGeometry {
  id: number;
  position: [number, number];
  type: GeometryTypes;
  refId: number;
  styling: GeometryStyle;
}

type RGBA = [number, number, number, number];

export interface GeometryStyle {
  text?: {
    font?: string;
    textAlign?: CanvasTextAlign;
    justify?: TextJustify;
    color?: RGBA;
  };
  backgroundColor?: RGBA;
  backgroundColorClicked?: RGBA;
}

interface State {
  fairMaps: FairMap[];
  last_load: number;
}

function generateMapGeometry(n: number, m: number, margin: number): MapGeometry[] {
  const geoms: MapGeometry[] = [];
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      geoms.push({
        id: i,
        position: [(i * margin) - (n * margin / 2), (j * margin) - (m * margin / 2)],
        type: "company",
        refId: 1+j+(i*m),
        styling: { backgroundColor: [255, 0, 0, 1] },
      });
    }
  }
  return geoms;
}

const example: FairMap = {
  id: 1,
  name: "FairMap",
  background: "SU1.png",
  styling: {
    mapSize: [1526, 678],
  },
  mapGeometry: generateMapGeometry(10,15, 50)
};

export const useFairMapsStore = defineStore("fairMaps", {
  state: (): State => ({
    fairMaps: [example],
    last_load: 0,
  }),
  actions: {
    fetchMaps(force = false) {
      return new Promise<void>((res, rej) => {
        if (force || this.last_load + NUMBER_OF_MS_BEFORE_RELOAD < Date.now()) {
          this.last_load = Date.now();

          this.axios
            .get("/v2/map/")
            .then((resp: any) => {
              this.fairMaps = resp.data;
              res();
            })
            .catch((err: any) => {
              rej(err);
            });
        }
      });
    },
  },
});
