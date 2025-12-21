import { RawGameMap } from "./map";
import { PromisedType } from "@/utils";
import { WebMapGenerator } from "@/public/game/pkg";

export const importMicropolisCoreWasmLib = async () => {
  const response = await fetch("/pkg/index.wasm");
  const buffer = await response.arrayBuffer;
  return WebAssembly.instantiate(buffer);
};

export type MicropolisCoreLib = PromisedType<
  ReturnType<typeof importMicropolisCoreWasmLib>
>;

export class MicropolisCoreLibConnector {
  constructor(private readonly coreLib: MicropolisCoreLib) {
    coreLib.main();
  }

  get versionInfo(): string {
    // TODO: export micropolis_core lib version from micropolis_core_wasm bindings
    return "v. alpha";
  }

  createNewMapGenerator(): WebMapGenerator {
    return this.coreLib.create_terrain_generator();
  }

  generateNewRandomMap(generator: WebMapGenerator, seed: number, width: number, height: number): RawGameMap {
    const generatedMap = this.coreLib.generate_new_map(generator, seed, width, height);
    return { map: generatedMap, seed };
  }
}

export const connectMicropolisCoreLib = (importedCoreLib: MicropolisCoreLib) => new MicropolisCoreLibConnector(importedCoreLib);
