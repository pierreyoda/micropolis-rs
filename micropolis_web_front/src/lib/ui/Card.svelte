<script lang="ts">
  import Button from "./Button.svelte";

  export let title: string;
  export let closable: boolean = false;
  export let extraClass: string | undefined = undefined;
  export let onClose: () => void | undefined = undefined;

  $: innerOnClose = onClose ? onClose : () => {};
  const baseClass = "flex flex-col items-center rounded";
  $: innerClass = extraClass ? `${baseClass} ${extraClass}` : baseClass;
</script>

<div class={innerClass}>
  <div class="flex items-center justify-between w-full px-6 py-3 rounded-t">
    <h3 class="font-bold text-gray-400">{title}</h3>
    {#if closable}
      <Button onClick={innerOnClose}>
        <span class="text-sm">X</span>
      </Button>
    {/if}
    <slot />
  </div>
</div>
