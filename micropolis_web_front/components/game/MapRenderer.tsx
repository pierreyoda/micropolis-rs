import "pixi-tilemap";
import React, { useEffect, useMemo, FunctionComponent } from "react";
import { Loader, LoaderResource, BaseRenderTexture, SCALE_MODES, RenderTexture } from "pixi.js";

interface MapRendererProps {
  width: number;
  height: number;
  loader: Loader;
  tilesImagePath?: string;
  onLoadingProgress: (loader: Loader, resource: LoaderResource) => void;
}

const TILE_SIZE = 16; // in pixels

const MapRenderer: FunctionComponent<MapRendererProps> = ({
  width,
  height,
  loader,
  onLoadingProgress,
  tilesImagePath = "/game/tiles.png",
}) => {
  useEffect(
    () => {
      loader
        .add("tiles", tilesImagePath)
        .on("progress", onLoadingProgress)
        .load();
    },
    [loader, onLoadingProgress],
  );

  const renderTexture = useMemo(
    () => {
      const base = new BaseRenderTexture({ width, height, scaleMode: SCALE_MODES.LINEAR });
      return new RenderTexture(base);
    },
    [width, height],
  );

  return (
    <div>test</div>
  );
};

export default MapRenderer;
