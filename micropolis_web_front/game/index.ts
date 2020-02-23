import { PromisedType } from "@/utils";

export const importMicropolisCoreWasmLib = async () =>
  import("../../micropolis_wasm/pkg");

export type MicropolisCoreLib = PromisedType<
  ReturnType<typeof importMicropolisCoreWasmLib>
>;

export class MicropolisCoreLibConnector {
  constructor(private readonly coreLib: MicropolisCoreLib) {
    coreLib.main();
  }

  get versionInfo(): string {
    // TODO: export micropolis_core lib version from micropolis_core_wasm bindings
    return this.coreLib.greet();
  }
}

export const connectMicropolisCoreLib = (importedCoreLib: MicropolisCoreLib) => new MicropolisCoreLibConnector(importedCoreLib);

export const gameVersionInfo = (gameLib: MicropolisCoreLib): string =>
  gameLib.greet();
