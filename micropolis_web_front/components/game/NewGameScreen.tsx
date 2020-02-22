import React, { FunctionComponent } from "react";
import { AppContext } from "react-pixi-fiber";
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
    <AppContext.Consumer>
      {(app: Application) => (<MapRenderer
        loader={app.loader}
      />)}
    </AppContext.Consumer>
  </Card>
);

export default NewGameScreen;
