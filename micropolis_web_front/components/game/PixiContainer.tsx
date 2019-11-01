import * as PIXI from "pixi.js";
import React, { useState, useRef, useLayoutEffect, FunctionComponent } from "react";


type PixiRendererType = "WebGL" | "canvas";

const getPixiSupportedType = (): PixiRendererType =>
  PIXI.utils.isWebGLSupported() ? "WebGL" : "canvas";

export interface PixiContainerProps {
  debug?: boolean;
}

const PixiContainer: FunctionComponent<PixiContainerProps> = (
  { debug } = { debug: false },
) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [, setApp] = useState<PIXI.Application>(null);

  // init pixi application
  useLayoutEffect(() => {
    if (debug) {
      PIXI.utils.sayHello(getPixiSupportedType());
    }
    const pixiApp = new PIXI.Application({
      width: 800,
      height: 600,
      antialias: true,
      transparent: false,
      backgroundColor: 0xadadad,
      resizeTo: containerRef.current,
      view: canvasRef.current,
    });
    setApp(pixiApp);
    return () => pixiApp.destroy(false);
  }, []);

  return (
    <div ref={containerRef} className="flex-grow w-full h-full">
      <canvas ref={canvasRef} />
    </div>
  );
};

export default PixiContainer;
