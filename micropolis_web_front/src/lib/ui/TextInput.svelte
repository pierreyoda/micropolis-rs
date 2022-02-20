<script lang="ts">
  export let value: string;
  export let placeholder: string | undefined;
</script>

<input
  {value}
  on:input={e => {
    value = e.target.value;
  }}
  class="custom-input"
  tabIndex={0}
  type="text"
  {placeholder}
/>
