import * as PIXI from "pixi.js";
import React, { useState, useRef, useLayoutEffect, FunctionComponent } from "react";

type PixiRendererType = "WebGL" | "canvas";

const getPixiSupportedType = (): PixiRendererType =>
  PIXI.utils.isWebGLSupported() ? "WebGL" : "canvas";

export interface PixiContainerProps {
  debug?: boolean;
}

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
  const [, setApp] = useState<PIXI.Application>(null);

  useLayoutEffect(() => {
    if (debug) {
      PIXI.utils.sayHello(getPixiSupportedType());
    }
  }, [debug]);

  // init pixi.js application
  useLayoutEffect(() => {
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
    return () => pixiApp.destroy(false);
  }, []);

  return (
    <div ref={containerRef} className="flex-grow w-full h-full">
      <canvas ref={canvasRef} className="absolute block" />
    </div>
  );
};

export default PixiContainer;
