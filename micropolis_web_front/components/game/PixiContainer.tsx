import React, { useState, useRef, useLayoutEffect, FunctionComponent } from "react";
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
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [, setApp] = useState<PIXI.Application>();

  useLayoutEffect(() => {
    if (debug) {
      utils.sayHello(getPixiSupportedType());
    }
  }, [debug]);

  // init pixi.js application
  useLayoutEffect(() => {
    if (!containerRef.current || !canvasRef.current) {
      return () => null;
    }
    const pixiApp = new PIXI.Application({
      width: 800,
      height: 600,
      antialias: true,
      transparent: false,
      backgroundColor: 0x000000,
      resizeTo: containerRef.current,
      view: canvasRef.current,
    });
    setApp(pixiApp);
    loadAssets(pixiApp); // TODO: look into Suspence for seamless async loading
    return () => pixiApp.destroy(false);
  }, []);

  return (
    <Stage
      className="flex-grow w-full h-full"
      width={800}
      height={600}
      options={{
        antialias: true,
        transparent: false,
        backgroundColor: 0x000000,
      }}
    >
      <NewGameScreen />
    </Stage>
  );
};

export default PixiContainer;
