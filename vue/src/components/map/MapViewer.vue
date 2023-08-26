<template>
  <div ref="mapRoot" class="map"></div>
</template>

<script setup lang="ts">
import Layer from "ol/layer/Layer.js";
import { Vector as VectorLayer } from "ol/layer.js";
import Map from "ol/Map.js";
import View from "ol/View.js";
import Feature from "ol/Feature.js";
import { LineString, Point } from "ol/geom.js";
import { Vector as VectorSource } from "ol/source.js";
import { composeCssTransform } from "ol/transform.js";
import { onMounted, ref } from "vue";
import "ol/ol.css";
import axios from "@/plugins/axios";

const mapRoot = ref<string | HTMLElement | undefined>(undefined);

const svgContainer = document.createElement("div");
svgContainer.classList.add("svgContainer");

const company_points = [
  new Feature(
    new LineString([
      [-1, -1],
      [1, 1],
    ])
  )
];
axios
  .get("/v2/image/Alten_Complete_original_colour_1de005d3ba579871270c897abd025912.svg")
  .then((resp: any) => {
    const svg = resp.request.responseXML.documentElement;
    svgContainer.ownerDocument.importNode(svg);
    svgContainer.appendChild(svg);
  })
  .catch((err: any) => {
    console.log("shit broke!", err);
  });

const width = 500;
const height = 1280;
const svgResolution = 360 / width;
svgContainer.style.width = width + "px";
svgContainer.style.height = height + "px";
svgContainer.style.transformOrigin = "top left";
svgContainer.className = "svg-layer";


const svgLayer = new Layer({
  render: function (frameState) {
    const scale = svgResolution / frameState.viewState.resolution;
    const center = frameState.viewState.center;
    const size = frameState.size;
    const cssTransform = composeCssTransform(
      size[0] / 2,
      size[1] / 2,
      scale,
      scale,
      frameState.viewState.rotation,
      -center[0] / svgResolution - width / 2,
      center[1] / svgResolution - height / 2
    );
    svgContainer.style.transform = cssTransform;
    svgContainer.style.opacity = this.getOpacity();
    return svgContainer;
  },
  zIndex: 2,
});

const vecLayer = new VectorLayer({
    source: new VectorSource({
      features: company_points,
    }),
    style: {
      "stroke-width": 20,
      "fill-color": [0, 0, 255, 0.8],
      "stroke-color": [0, 0 , 0, 1],
    },
    zIndex: 4
  })


const map = new Map({
  layers: [vecLayer, svgLayer],
  view: new View({
    center: [0, 0],
    extent: [-180, -180, 180, 180],
    projection: "EPSG:4326",
    zoom: 2,
  }),
});
console.log(map)

onMounted(() => {
  map.setTarget(mapRoot?.value);
});
</script>

<style>
.map {
  width: 500px;
  height: 400px;
}

#map {
  background: black;
}

.svg-layer path:hover {
  opacity: 0.4;
}
</style>
