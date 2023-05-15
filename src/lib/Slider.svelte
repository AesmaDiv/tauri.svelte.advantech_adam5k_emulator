<script lang="ts">
  import { slide } from "svelte/transition";
  import type { SlideParams } from "svelte/transition"
  import { linear } from 'svelte/easing';
  let options : SlideParams = {duration: 250, easing: linear, axis: 'x'};

  export let header : string = "Заголовок";
  export let group : string = "Any";

  let root : HTMLDivElement;
  const onSwitch = () => group = header;

  $: state = (() => {
    let result = group === header;
    !!root && root.style.setProperty("--background", result ? "aqua" : "white");
    return result;
  })();
</script>

<div class="root" bind:this={root}>
  <div class="header-container" on:mousedown={onSwitch}>
    <div class="header">{header}</div>
  </div>
  {#if state}
  <div class="container" transition:slide={options}>
    <slot/>
  </div>
  {/if}
</div>

<style>
  .root {
    color: black;
    background-color: var(--background);
    border-radius: 5px;
    box-shadow: 0px 5px 10px black;
    display: flex;
    flex-direction: row;
    margin: 13px 3px;
    transition: background-color 500ms;
  }
  .header-container {
    width: 2em;
    height: 100%;
    cursor: pointer;
    white-space: nowrap;
    cursor: pointer;
  }
  .header {
    font-size: large;
    margin-top: 1em;
    transform-origin: 0 0;
    transform: rotate(90deg) translateY(-1.5em);
    user-select: none;
  }
  .container {
    padding: 10px;
  }
</style>