import type { RawGameMap } from "./map";
import type { WebMapGenerator } from "../../../micropolis_wasm/pkg";
// import init, { main, create_terrain_generator, generate_new_map} from "micropolis_wasm";
import init, { create_terrain_generator, generate_new_map } from "../../../micropolis_wasm/pkg";

export class MicropolisCoreLibConnector {
  async init() {
    return init();
  }

  get versionInfo(): string {
    // TODO: export micropolis_core lib version from micropolis_core_wasm bindings
    return "v. alpha";
  }

  createNewMapGenerator(): WebMapGenerator {
    return create_terrain_generator();
  }

  generateNewRandomMap(generator: WebMapGenerator, seed: number, width: number, height: number): RawGameMap {
    const generatedMap = generate_new_map(generator, seed, width, height);
    return { map: generatedMap, seed };
  }
}
