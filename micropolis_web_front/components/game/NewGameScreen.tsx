import React, { FunctionComponent } from "react";

import Card from "../common/Card";
import MapRenderer from "./MapRenderer";

export interface NewGameScreenProps {
}

const NewGameScreen: FunctionComponent<NewGameScreenProps> = () => (
  <Card
    title={"New Game"}
    backgroundColor="#edad0a"
  >
    
  </Card>
);

export default NewGameScreen;
