import React, { FunctionComponent } from "react";
import { AppContext, Stage } from "react-pixi-fiber";
import { Application } from "pixi.js";

import Card from "../common/Card";
import MapRenderer from "./MapRenderer";

export interface NewGameScreenProps {
}

const NewGameScreen: FunctionComponent<NewGameScreenProps> = () => (
  <Card
    title={"New Game"}
    backgroundColor="#edad0a"
  >
    <div className="flex items-start justify-between">
      <Stage
        className="flex-grow w-full h-auto"
        options={{
          width: 800,
          height: 800,
          antialias: true,
          transparent: false,
          sharedTicker: true,
          backgroundColor: 0x22543d,
          resolution: window.devicePixelRatio || 1,
        }}
      >
        {/* <AppContext.Consumer>
          {(app: Application) => (<MapRenderer
              loader={app.loader}
              renderer={app.renderer}
              tilesImagePath="/game/tiles.png"
              onLoadingProgress={() => {}}
          />)}
        </AppContext.Consumer> */}
      </Stage>
    </div>
  </Card>
);

export default NewGameScreen;
