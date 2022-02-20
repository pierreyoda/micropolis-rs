<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Button from "$lib/ui/Button.svelte";
  import type { GameMap } from "../../game/map";
  import { getRandomInteger } from "../../utils"; // TODO: alias
  import { MicropolisCoreLibConnector } from "../../game"; // TODO: alias
  import TextInput from "$lib/ui/TextInput.svelte";
  import MapRenderer from "./MapRenderer.svelte";

  interface GeneratedGameMap {
    seed: number;
    gameMap: GameMap;
  }

  const gameLib = new MicropolisCoreLibConnector();
  let generateNewMap = () => {};
  let currentlyViewedMapIndex = 0;
  let generatedMaps: readonly GeneratedGameMap[] = [];
  gameLib.init().then(() => {
    const generator = gameLib.createNewMapGenerator();
    const generateSeed = (): number => getRandomInteger(123, 123456);
    generateNewMap = () => {
      const rawMap = gameLib.generateNewRandomMap(generator, generateSeed(), 120, 100);
      console.log(rawMap);
    };
    generateNewMap();
  });
  $: currentGeneratedMap = generatedMaps[currentlyViewedMapIndex];

  let cityName = "";
  $: isCityNameValid = cityName.trim().length > 0;
</script>

<div class="flex items-start justify-center">
  {#if currentGeneratedMap}
    <div class="flex flex-col mr-12">
      <p class="mb-4 text-center text-gray-700">Seed: {currentGeneratedMap.seed}</p>
      <div class="border-4 border-gray-500">
        <MapRenderer scale={0.2} map={currentGeneratedMap.gameMap} />
      </div>
      <div class="flex items-center justify-between w-full mt-4">
        <Button
          disabled={currentlyViewedMapIndex === 0}
          onClick={() => {
            --currentlyViewedMapIndex;
          }}
        >
          Previous
        </Button>
        <p class="text-center text-gray-700">
          {currentlyViewedMapIndex + 1} / {generatedMaps.length}
        </p>
        <Button
          disabled={currentlyViewedMapIndex >= generatedMaps.length - 1}
          onClick={() => {
            ++currentlyViewedMapIndex;
          }}
        >
          Next
        </Button>
      </div>
    </div>
  {/if}
  <Card title="New Game" extraClass="justify-between">
    <TextInput bind:value={cityName} placeholder="City name (mandatory)" />
    <div class="flex flex-col w-full">
      <Button onClick={generateNewMap} extraClass="w-full mt-10">Generate</Button>
    </div>
    {#if currentGeneratedMap}
      <Button disabled={!isCityNameValid} onClick={() => {}} extraClass="w-full mt-4 bg-green-500">
        Play this map
      </Button>
    {/if}
  </Card>
</div>
