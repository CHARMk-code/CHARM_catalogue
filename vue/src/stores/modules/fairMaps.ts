import type { TextJustify } from "ol/style/Text"
import { defineStore } from "pinia"

const NUMBER_OF_MS_BEFORE_RELOAD = 60000 // Don't reload more often then ones an hour.

export interface FairMap {
  id: number
  name: string
  background: string
  styling: FairMapStyle
  mapGeometry: MapGeometry[]
}

export interface FairMapStyle {
  circleSize?: number
  maxZoom?: number
  mapSize: [number, number]
}

export type GeometryTypes = "company" | "other"

export interface MapGeometry {
  id: number
  position: [number, number]
  type: GeometryTypes
  refId: number
  text?: string
  styling: GeometryStyle
}

type RGBA = [number, number, number, number]

export interface GeometryStyle {
  text?: {
    font?: string
    textAlign?: CanvasTextAlign
    justify?: TextJustify
    color?: RGBA
  }
  backgroundColor?: RGBA
  backgroundColorClicked?: RGBA
}

export interface MapState {
  selectedMap: number | undefined
  selectedMarker:
    | {
        refId: number
        newPosition: [number, number]
      }
    | undefined
}

interface State {
  fairMaps: Map<number, FairMap>
  currentState: MapState
  last_load: number
}

function generateMapGeometry(
  n: number,
  m: number,
  margin: number,
): MapGeometry[] {
  const geoms: MapGeometry[] = []
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      geoms.push({
        id: i,
        position: [
          i * margin - (n * margin) / 2,
          j * margin - (m * margin) / 2,
        ],
        type: "company",
        refId: 1 + j + i * m,
        styling: { backgroundColor: [255, 0, 0, 1] },
      })
    }
  }
  return geoms
}

const example: FairMap = {
  id: 1,
  name: "FairMap",
  background: "SU1.png",
  styling: {
    mapSize: [1526, 678],
  },
  mapGeometry: generateMapGeometry(10, 15, 50),
}

export const useFairMapsStore = defineStore("fairMaps", {
  state: (): State => ({
    fairMaps: new Map(),
    currentState: {
      selectedMap: undefined,
      selectedMarker: undefined,
    },
    last_load: 0,
  }),
  actions: {
    fetchMaps(force = false) {
      return new Promise<void>((res, rej) => {
        if (force || this.last_load + NUMBER_OF_MS_BEFORE_RELOAD < Date.now()) {
          this.axios
            .get("/v2/map/")
            .then((resp: any) => {
              resp.data.map((fairMap: any) => {
                this.fairMaps.set(fairMap.id, {
                  id: fairMap.id,
                  name: fairMap.name,
                  background: fairMap.background,
                  styling: fairMap.map_data.styling ?? { mapSize: [100, 100] },
                  mapGeometry: fairMap.map_data.mapGeometry ?? [],
                })

              })

              this.currentState.selectedMap = Math.min(...this.fairMaps.keys())
              res()
            })
            .catch((err: any) => {
              rej(err)
            })
        }

        this.last_load = Date.now()
      })
    },
    saveFairMap(fairMapId: number) {
      return new Promise((resolve, reject) => {
        const fairMap = this.fairMaps.get(fairMapId)
        if (fairMap) {
          const data = {
            id: fairMap.id,
            name: fairMap.name,
            background: fairMap.background,
            map_data: {
              styling: fairMap.styling,
              mapGeometry: fairMap.mapGeometry,
            },
          }
          
          const result = this.axios.put("/v2/map/", data)

          resolve(result)
        } else {
          return reject("fairMap undefined")
        }
      })
    },
    findMarker(
      fairMapId: number,
      markerRefId: number,
    ): MapGeometry | undefined {
      return this.fairMaps
        .get(fairMapId)
        ?.mapGeometry.find(({ refId }) => refId === markerRefId)
    },
    removeMarker(fairMapId: number, markerRefId: number) {
      const fairMap = this.fairMaps.get(fairMapId)
      if (fairMap) {
        const removeIndex = fairMap.mapGeometry.findIndex(
          ({ refId }) => refId === markerRefId,
        )
        if (removeIndex !== -1) fairMap.mapGeometry.splice(removeIndex, 1)
      }
    },
    addOrReplaceMarker(fairMapId: number, marker: MapGeometry) {
      const fairMap = this.fairMaps.get(fairMapId)
      if (fairMap) {
        const index = fairMap.mapGeometry.findIndex(
          ({ refId }) => refId === marker.refId,
        )
        if (index !== -1) fairMap.mapGeometry.splice(index, 1, marker)
        else fairMap.mapGeometry.push(marker)
      }
    },
  },
})
