import { PromisedType } from "@/utils";
import { WebMapGenerator } from "@/pkg/";

export const importMicropolisCoreWasmLib = async () => import("@/pkg");

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

  generateNewRandomMap(generator: WebMapGenerator, width: number, height: number) {
    return this.coreLib.generate_new_map(generator, width, height);
  }
}

export const connectMicropolisCoreLib = (importedCoreLib: MicropolisCoreLib) => new MicropolisCoreLibConnector(importedCoreLib);
