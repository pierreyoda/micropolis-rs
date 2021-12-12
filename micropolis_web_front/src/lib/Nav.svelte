<nav>
  <h1 class="text-2xl">
    <a sveltekit:prefetch href="/">micropolis-rs</a>
  </h1>
  <ul class="flex items-center">
    <li class="mr-8">
      <a sveltekit:prefetch href="/about">About</a>
    </li>
    <li>
      <a href="https://github.com/pierreyoda/micropolis-rs" rel="noopener noreferrer" target="_blank">GitHub</a>
    </li>
  </ul>
</nav>

<style lang="postcss">
  nav {
    @apply flex items-center justify-between w-full px-2 py-4;
    @apply font-bold text-white bg-green-900;
  }
</style>
