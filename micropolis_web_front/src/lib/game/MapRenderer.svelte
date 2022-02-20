<script lang="ts">
  import Tile from "./Tile.svelte";
  import type { GameMap } from "../../game/map";

  export let map: GameMap;
  /** Between 0 and 1. */
  export let scale: number | undefined = undefined;
</script>

<div class="flex">
  {#each map.tiles as column}
    <div class="flex-col">
      {#each column as tile}
        <Tile scale={scale ?? 1} tileIndex={tile.tileType >= 0 ? tile.tileType : 0} atlasImageUrl="/games/tiles.png" />
      {/each}
    </div>
  {/each}
</div>
