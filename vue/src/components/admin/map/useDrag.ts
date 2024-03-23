import { useFairMapsStore } from "@/stores/modules/fairMaps"
import { Coordinate } from "ol/coordinate"
import type { FeatureLike } from "ol/Feature"
import { Geometry } from "ol/geom"
import { Pointer as PointerInteraction } from "ol/interaction.js"
import type MapBrowserEvent from "ol/MapBrowserEvent.js"

const fairMapsStore = useFairMapsStore()

///////////////////////////////////
// Handling of dragging objects
export class Drag extends PointerInteraction {
  coordinate_: Coordinate | null
  cursor_: string | undefined
  feature_: FeatureLike | null
  previousCursor_: string | undefined
  constructor() {
    super({
      handleDownEvent: handleDownEvent,
      handleDragEvent: handleDragEvent,
      handleMoveEvent: handleMoveEvent,
      handleUpEvent: handleUpEvent,
    })

    /**
     * @type {import("../src/ol/coordinate.js").Coordinate}
     * @private
     */
    this.coordinate_ = null

    /**
     * @type {string|undefined}
     * @private
     */
    this.cursor_ = "pointer"

    /**
     * @type {Feature}
     * @private
     */
    this.feature_ = null

    /**
     * @type {string|undefined}
     * @private
     */
    this.previousCursor_ = undefined
  }
}

function handleDownEvent(
  this: Drag,
  evt: MapBrowserEvent<PointerEvent>,
): boolean {
  const map = evt.map

  const feature = map.forEachFeatureAtPixel(
    evt.pixel,
    function (feature: FeatureLike) {
      return feature
    },
  )

  const properties = feature?.getProperties()
  const isActiveMarker =
    properties?.refId === fairMapsStore.currentState.selectedMarker?.refId

  if (feature && isActiveMarker) {
    this.coordinate_ = evt.coordinate
    this.feature_ = feature
  }

  return !!feature && isActiveMarker
}

function handleDragEvent(this: Drag, evt: MapBrowserEvent<PointerEvent>) {
  const [evtX, evtY] = evt.coordinate
  const [thisX, thisY] = this.coordinate_ ?? []

  if (evtX && evtY && thisX && thisY) {
    const deltaX = evtX - thisX
    const deltaY = evtY - thisY

    const feature = this.feature_
    if (feature === null) throw "No feature"

    const geometry = feature.getGeometry() as Geometry

    geometry.translate(deltaX, deltaY)

    fairMapsStore.currentState.selectedMarker = {
      refId: feature.getProperties().refId,
      newPosition: feature.getProperties().geometry.flatCoordinates.slice(0, 2),
    }

    this.coordinate_ = evt.coordinate
  }
}

function handleMoveEvent(this: Drag, evt: MapBrowserEvent<PointerEvent>) {
  if (this.cursor_) {
    const map = evt.map
    const feature = map.forEachFeatureAtPixel(evt.pixel, function (feature) {
      return feature
    })
    const element = evt.map.getTargetElement()
    if (feature) {
      if (element.style.cursor != this.cursor_) {
        this.previousCursor_ = element.style.cursor
        element.style.cursor = this.cursor_
      }
    } else if (this.previousCursor_ !== undefined) {
      element.style.cursor = this.previousCursor_
      this.previousCursor_ = undefined
    }
  }
}

function handleUpEvent(this: Drag): boolean {
  this.coordinate_ = null
  this.feature_ = null
  return false
}
