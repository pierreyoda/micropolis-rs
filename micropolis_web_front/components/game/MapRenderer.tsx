import "pixi-tilemap";
import { CustomPIXIComponentBehaviorDefinition, CustomPIXIComponent } from "react-pixi-fiber";
import { Loader, LoaderResource, DisplayObject, Renderer } from "pixi.js";

export interface MapRendererProps {
  renderer: Renderer;
  tilesImagePath: string;
  loader: Loader;
  onLoadingProgress: (loader: Loader, resource: LoaderResource) => void;
}

const TILE_SIZE = 16; // in pixels
const ATLAS_ROWS = 16;
const ATLAS_COLUMNS = 60;
const ATLAS_TILES_COUNT = ATLAS_ROWS * ATLAS_COLUMNS;

let tilemap: PIXI.tilemap.CompositeRectTileLayer;

const MapRenderer: CustomPIXIComponentBehaviorDefinition<DisplayObject, MapRendererProps> = {
  customDisplayObject: ({ loader, tilesImagePath, onLoadingProgress }) => {
    loader
      .add("tiles", tilesImagePath)
      .on("progress", onLoadingProgress)
      .load((loader, resources) => {
        tilemap = new PIXI.tilemap.CompositeRectTileLayer(0, [resources["tiles"]?.texture!]);
        for (let tileIndex = 0; tileIndex < ATLAS_TILES_COUNT; tileIndex++) {
          tilemap.addFrame(`tile-${tileIndex}`, tileIndex % ATLAS_ROWS * TILE_SIZE, tileIndex/ ATLAS_ROWS * TILE_SIZE);
        }
      });
    tilemap.visible = true;
    return tilemap;
  },
  // customApplyProps: (map, { renderer }) => {
  //   map.render(renderer)
  // },
};

export default CustomPIXIComponent(MapRenderer, "MAP_RENDERER");
