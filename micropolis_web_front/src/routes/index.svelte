<script lang="ts">
  import NewGameScreen from "$lib/game/NewGameScreen.svelte";

  export const ssr = false;
</script>

<div class="game-container">
  <!-- TODO: loading -->
  <NewGameScreen />
</div>
