import React, { FunctionComponent, useLayoutEffect } from "react";
import { Stage } from "react-pixi-fiber";
import { utils } from "pixi.js";

import NewGameScreen from "./NewGameScreen";

type PixiRendererType = "WebGL" | "canvas";

const getPixiSupportedType = (): PixiRendererType => utils.isWebGLSupported() ? "WebGL" : "canvas";

export interface PixiContainerProps {
  debug?: boolean;
}

const loadAssets = (app: PIXI.Application) => {
  app.loader.add("tile_map", "/game/tiles.png");
};

/**
 * A PixiJS Container, *i.e.* the root container of the graphical application.
 *
 * It will instantiate either a full-blown WebGL context, or can seamlessly fall back
 * to a better widely-supported HTML5 canvas.
 *
 * @see http://pixijs.download/release/docs/PIXI.Application.html#stage
 */
const PixiContainer: FunctionComponent<PixiContainerProps> = (
  { debug } = { debug: false },
) => {
  useLayoutEffect(() => {
    if (debug) {
      utils.sayHello(getPixiSupportedType());
    }
  }, [debug]);

  return (
    <Stage
      className="flex-grow w-full h-auto"
      options={{
        width: 800,
        height: 600,
        antialias: true,
        transparent: false,
        backgroundColor: 0x22543d,
      }}
    >
      <NewGameScreen />
    </Stage>
  );
};

export default PixiContainer;
