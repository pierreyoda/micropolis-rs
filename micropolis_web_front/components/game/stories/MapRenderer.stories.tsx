import React from "react";
import { Application } from "pixi.js";
import { storiesOf } from "@storybook/react";
import { withKnobs } from "@storybook/addon-knobs";
import { Stage, AppContext } from "react-pixi-fiber";

import MapRenderer from "../MapRenderer";
import StoryWrapper from "@/components/common/stories/StoryWrapper";

// storiesOf("Game/Map", module)
//   .addDecorator(withKnobs)
//   .add("Tilemap", () => {
//     return (
//       <StoryWrapper>
//         <Stage
//           className="flex-grow w-full h-auto"
//           options={{
//             width: 800,
//             height: 600,
//             antialias: true,
//             transparent: false,
//             backgroundColor: 0x22543d,
//           }}
//         >
//           <AppContext.Consumer>
//             {(app: Application) => (<MapRenderer
//                 loader={app.loader}
//                 renderer={app.renderer}
//                 tilesImagePath="/game/tiles.png"
//                 onLoadingProgress={() => {}}
//             />)}
//           </AppContext.Consumer>
//         </Stage>
//       </StoryWrapper>
//     );
//   });
