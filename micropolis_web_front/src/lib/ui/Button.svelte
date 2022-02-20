<script lang="ts">
  export let onClick: () => void;
  export let disabled = false;
  export let extraClass: string | undefined = undefined;

  $: innerOnClick = disabled ? () => {} : onClick;
  $: innerClass = extraClass ? `custom-button ${extraClass}` : "custom-button";
</script>

<button class={innerClass} onClick={innerOnClick} {disabled}>
  <slot />
</button>
