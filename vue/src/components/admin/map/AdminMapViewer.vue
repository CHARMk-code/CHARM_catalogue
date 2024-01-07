<template>
  <div ref="mapRoot" class="map" />
</template>

<script setup lang="ts">
import { Vector as VectorLayer } from "ol/layer.js"
import Map from "ol/Map.js"
import View from "ol/View.js"
import Feature, { type FeatureLike } from "ol/Feature.js"
import { Circle, Geometry } from "ol/geom.js"
import { Vector as VectorSource } from "ol/source.js"
import { computed, onMounted, ref, watch } from "vue"
import "ol/ol.css"
import Select from "ol/interaction/Select.js"
import { click } from "ol/events/condition.js"
import axios from "@/plugins/axios"
import ImageLayer from "ol/layer/Image"
import Static from "ol/source/ImageStatic"
import Projection from "ol/proj/Projection"
import { Style, Fill, Text } from "ol/style.js"
import type {
  FairMap,
  FairMapStyle,
  GeometryStyle,
  MapGeometry,
} from "@/stores/modules/fairMaps"
import { useFairMapsStore } from "@/stores/modules/fairMaps"
import { useCompaniesStore } from "@/stores/modules/companies"
import { Drag } from "./useDrag"

const props = defineProps<{
  fairMapId: number
  selectedMarker: MapGeometry | undefined
  enableDrag?: boolean
}>()

const emit = defineEmits<{
  "update:selectedMarker": [marker: MapGeometry | undefined]
}>()

///////////////////////
// Handle map creation
const mapRoot = ref<string | HTMLElement | undefined>(undefined)
const companiesStore = useCompaniesStore()
const fairMapsStore = useFairMapsStore()

//Create empty map and return it
function setupMap(): Map {
  return new Map()
}

// Initializes map with information from the store
function InitializeMap(fairMap: FairMap) {
  console.log(fairMap)
  emptyMap.value = false
  const viewExtent = extentFromSize(fairMap.styling.mapSize.map((n) => n * 2))
  const viewProjection = new Projection({
    code: "FairMap",
    units: "pixels",
    extent: viewExtent,
  })

  const backgroundLayer = createBackgroundLayer(fairMap)

  const companyMarkers = fairMap.mapGeometry.filter(
    (geom: MapGeometry) => geom.type == "company",
  )

  const companyLayer = createCompanyLayer(companyMarkers, fairMap.styling)

  map.value.setLayers([backgroundLayer, companyLayer])
  map.value.setView(
    new View({
      center: [0, 0],
      extent: viewExtent,
      projection: viewProjection,
      zoom: 1.5, // TODO: Define scale factor depending on background sizing
      maxZoom: fairMap.styling.maxZoom || 4,
    }),
  )

  const clickSelect = new Select({
    condition: click,
    style: selectedMarkerStyle,
  })

  map.value.addInteraction(clickSelect)
  clickSelect.on("select", (e) => {
    const fairMapId = fairMapsStore.currentState.selectedMap ?? 0

    e.deselected.forEach((feature) => {
      feature.setStyle(defaultMarkerStyle)

      fairMapsStore.currentState.selectedMarker = undefined
      emit("update:selectedMarker", undefined)
    })

    e.selected.forEach((feature) => {
      const refId = feature.getProperties().refId
      const newPosition = feature
        .getProperties()
        .geometry.flatCoordinates.slice(0, 2)

      // this will be overwritten if multiple markers are selected simultaneously
      fairMapsStore.currentState.selectedMarker = { refId, newPosition }

      const selectedMarker = fairMapsStore.findMarker(fairMapId, refId)

      if (selectedMarker) {
        console.log("saving New position")
        selectedMarker.position = newPosition
      }
      emit("update:selectedMarker", selectedMarker)
    })

    if (props.enableDrag) {
      e.selected.concat(e.deselected).forEach((feature) => {
        const refId = feature.getProperties().refId
        const newPosition = feature
          .getProperties()
          .geometry.flatCoordinates.slice(0, 2)

        const savedMarker = fairMapsStore.findMarker(fairMapId, refId)

        if (savedMarker) {
          savedMarker.position = newPosition
        }
      })
    }
  })

  if (props.enableDrag) map.value.addInteraction(new Drag())
}

function createBackgroundLayer(fairmap: FairMap): ImageLayer<Static> {
  const extent = extentFromSize(fairmap.styling.mapSize)
  const projection = new Projection({
    code: "FairMap",
    units: "pixels",
    extent: extent,
  })

  return new ImageLayer({
    source: new Static({
      url: axios.defaults.baseURL + "/v2/image/" + fairmap.background,
      projection: projection,
      imageExtent: extent,
    }),
  })
}

function createCompanyLayer(
  companyMarkers: MapGeometry[],
  mapStyling: FairMapStyle,
): VectorLayer<VectorSource<Feature<Geometry>>> {
  const features = companyMarkers.map((geom) => createMarker(geom, mapStyling))

  return new VectorLayer({
    source: new VectorSource({
      features,
    }),
    updateWhileAnimating: true,
    updateWhileInteracting: true,
  })
}

function extentFromSize(size: number[]): [number, number, number, number] {
  if (!size[0] || !size[1]) throw "invalid input size"

  return [-size[0] / 2, -size[1] / 2, size[0] / 2, size[1] / 2]
}

function selectedMarkerStyle(feature: FeatureLike, resolution: number): Style {
  const styling: GeometryStyle = feature.get("geomStyle")
  const mapStyling: FairMapStyle = feature.get("mapStyle")

  let backgroundColorClicked
  if (styling.backgroundColorClicked) {
    backgroundColorClicked = styling.backgroundColorClicked
  } else if (styling.backgroundColor) {
    backgroundColorClicked = [
      Math.max(styling.backgroundColor[0] - 80, 0),
      Math.max(styling.backgroundColor[1] - 80, 0),
      Math.max(styling.backgroundColor[2] - 80, 0),
      styling.backgroundColor[3],
    ]
  } else {
    backgroundColorClicked = [15, 15, 15, 1]
  }

  return new Style({
    text: new Text({
      font: styling.text?.font || "bold 22px sans-serif",
      textAlign: styling.text?.textAlign || "center",
      justify: styling.text?.justify || "center",
      text: feature.get("text"),

      scale: (mapStyling.circleSize || 24) / 24 / resolution,
      fill: new Fill({
        color: styling.text?.color || [0, 0, 0, 1],
      }),
    }),
    fill: new Fill({
      color: backgroundColorClicked,
    }),
  })
}

function defaultMarkerStyle(feature: FeatureLike, resolution: number): Style {
  const styling: GeometryStyle = feature.get("geomStyle")
  const mapStyling: FairMapStyle = feature.get("mapStyle")

  return new Style({
    text: new Text({
      font: styling.text?.font || "bold 22px sans-serif",
      textAlign: styling.text?.textAlign || "center",
      justify: styling.text?.justify || "center",
      text: feature.get("text"),

      scale: (mapStyling.circleSize || 24) / 24 / resolution,
      fill: new Fill({
        color: styling.text?.color || [0, 0, 0, 1],
      }),
    }),
    fill: new Fill({ color: styling.backgroundColor || "lightgray" }),
  })
}

function createMarker(geom: MapGeometry, mapStyling: FairMapStyle) {
  const styling = geom.styling

  const company = companiesStore.companies.get(geom.refId)

  const text = company?.booth_number.toString()

  const feature = new Feature({
    geometry: new Circle(geom.position, mapStyling.circleSize || 24, "XY"), // TODO: Define scale factor depending on background sizing
  })

  feature.setId(geom.refId)

  feature.setStyle(defaultMarkerStyle)

  feature.set("refId", geom.refId)
  feature.set("geomStyle", styling)
  feature.set("mapStyle", mapStyling)
  feature.set("text", text)
  return feature
}

watch(
  () => props.selectedMarker,
  () => {
    if (props.selectedMarker) {
      const vectorSource = map.value
        .getAllLayers()[1]!
        .getSource() as VectorSource

      const existingFeature = vectorSource.getFeatureById(
        props.selectedMarker.id,
      ) as Feature<Geometry> | null

      console.log("existingFeature", existingFeature)
      if (existingFeature) {
        existingFeature.setStyle(selectedMarkerStyle)
      } else {
        const mapStyling = fairMapsStore.fairMaps.get(props.fairMapId)?.styling
        if (mapStyling) {
          vectorSource.addFeature(
            createMarker(props.selectedMarker, mapStyling),
          )
        }
      }

      console.log(vectorSource.getFeatures())
    }
  },
)

const fairMap = computed(() => {
  const fairMap = fairMapsStore.fairMaps.get(props.fairMapId)
  return fairMap
})

watch(fairMap, () => {
  if (emptyMap.value && fairMap.value) {
    InitializeMap(fairMap.value)
  }
})

watch(
  () =>
    fairMapsStore.fairMaps.get(fairMapsStore.currentState.selectedMap ?? -1),
  (newValue) => {
    if (newValue) {
      fairMapsStore.fairMaps.get(
        fairMapsStore.currentState.selectedMap!,
      )!.mapGeometry = []
      console.log(
        fairMapsStore.fairMaps.get(fairMapsStore.currentState.selectedMap!)!
          .mapGeometry,
      )
      InitializeMap(newValue)
    }
  },
)

const emptyMap = ref(true)
const map = ref<Map>(setupMap())

// watch(
//   () => [fairMap.value, fairMap.value?.mapGeometry, fairMap.value?.styling],
//   ([newFairMap, _newGeom, _newStyling]) => {
//     console.log("updating", newFairMap, _newGeom, _newStyling)
//     if (newFairMap) InitializeMap(newFairMap as FairMap)
//   },
// )

onMounted(() => {
  map.value.setTarget(mapRoot?.value)
  if (
    fairMapsStore.currentState.selectedMap &&
    fairMapsStore.fairMaps.get(fairMapsStore.currentState.selectedMap)
  ) {
    InitializeMap(
      fairMapsStore.fairMaps.get(fairMapsStore.currentState.selectedMap)!,
    )
  }
})
</script>

<style scoped lang="scss">
#mapLayoutContainer {
  height: calc(100vh - 90px);
}
</style>
