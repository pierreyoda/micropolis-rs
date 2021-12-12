<script lang="ts">
  import { getRandomInteger } from "../../utils"; // TODO: alias
  import { MicropolisCoreLibConnector } from "../../game"; // TODO: alias

  const gameLib = new MicropolisCoreLibConnector();
  const generator = gameLib.createNewMapGenerator();
  const generateSeed = (): number => getRandomInteger(123, 123456);
  const generateNewMap = () => {
    const rawMap = gameLib.generateNewRandomMap(generator, generateSeed(), 120, 100);
    console.log(rawMap);
  };

  let cityName = "";
  $: isCityNameValid = cityName.trim().length > 0;
</script>

<div class="flex items-start justify-center">TEST</div>
