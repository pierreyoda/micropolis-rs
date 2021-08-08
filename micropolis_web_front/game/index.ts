import { PromisedType } from "@/utils";
import { WebCityGenerator, WebCityGeneratorBuilder } from "@/pkg/";
import { RawGameMap } from "./map";

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

  createNewCityGeneratorBuilder(): WebCityGeneratorBuilder {
    return this.coreLib.create_city_generator_builder();
  }

  generateNewRandomMap(generator: WebCityGenerator, seed: number): RawGameMap {
    const generatedMap = generator.build_random_map(seed);
    return { handle: generatedMap, map: generatedMap.get_tiles(), seed };
  }
}

export const connectMicropolisCoreLib = (importedCoreLib: MicropolisCoreLib) => new MicropolisCoreLibConnector(importedCoreLib);
