<script lang="ts">
  import "../app.css";
  import Nav from "$lib/Nav.svelte";
</script>

<svelte:head>
  <meta charSet="UTF-8" lang="en" />
  <title>micropolis-rs</title>
  <meta name="description" content="Rewrite of Micropolis (the classic, open-source Sim City) in Rust and Typescript" />
  <!-- <link rel="icon" sizes="192x192" href="/touch-icon.png" />
  <link rel="apple-touch-icon" href="/touch-icon.png" />
  <link rel="mask-icon" href="/favicon-mask.svg" color="#49B882" />
  <link rel="icon" href="/favicon.ico" />
  <meta property="og:url" content={url} />
  <meta property="og:title" content="micropolis-rs" />
  <meta property="og:description" content="Rewrite of Micropolis (the classic, open-source Sim City) in Rust and Typescript" />
  <meta name="twitter:site" content={url} />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:image" content={ogImage} />
  <meta property="og:image" content={ogImage} />
  <meta property="og:image:width" content="1200" />
  <meta property="og:image:height" content="630" /> -->
</svelte:head>
<div class="flex flex-col w-screen h-screen">
  <Nav />
  <main class="flex-grow">
    <slot />
  </main>
</div>
