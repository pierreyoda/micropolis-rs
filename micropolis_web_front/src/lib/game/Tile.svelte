<script lang="ts">
  /** In pixels. */
  export const TILE_SIZE = 16;
  const ATLAS_ROWS = 16;
  const ATLAS_COLUMNS = 64;

  export let tileIndex: number;
  export let atlasImageUrl: string;
  export let scale: number;

  $: atlasX = (tileIndex % ATLAS_ROWS) * TILE_SIZE;
  $: atlasY = (tileIndex / ATLAS_ROWS) * TILE_SIZE;
  $: style = Object.entries({
    width: TILE_SIZE * scale,
    height: TILE_SIZE * scale,
    objectFIt: "none",
    objectPosition: `-${atlasX}px -${atlasY}px`,
  }).reduce((acc, [key, value]) => `${acc}${key}:${value};`, "");
</script>

<!-- svelte-ignore a11y-missing-attribute -->
<img src={atlasImageUrl} {style} class="not-selectable" />
