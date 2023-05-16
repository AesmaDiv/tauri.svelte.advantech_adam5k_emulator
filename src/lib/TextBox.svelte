<script lang="ts">
  import { onMount } from "svelte";
  import { getInheritedColor, getInheritedBackgroundColor } from "../shared/common";

  export let title: string = "Заголовок";
  export let value: string = "";

  let root: HTMLDivElement;
  let label: HTMLLabelElement;
  let input: HTMLInputElement;
  let select: HTMLSelectElement;
  let textarea: HTMLTextAreaElement;

  const labelTop = ['.3em', '-0.6em'];
  const labelLeft = ['.3em', '.8em'];
  const labelSize = ['100%', '60%'];
  const labelPadding = ['0.07em', '0.07em .2em'];
  const labelOpacity = ['30%', '100%']

  const change = () => { return value; }

  onMount(() => {
    $$props.backgroundColor ??= getInheritedBackgroundColor(root);
    $$props.color ??= getInheritedColor(root);
  })

  function onFocus(event: MouseEvent) {
    if (value) return;
    const ind = { mouseleave: 0, mouseenter: 1}[event.type];
    label.style.setProperty("--labelTop", labelTop[ind]);
    label.style.setProperty("--labelLeft", labelLeft[ind]);
    label.style.setProperty("--labelSize", labelSize[ind]);
    label.style.setProperty("--labelPadding", labelPadding[ind]);
    label.style.setProperty("--labelOpacity", labelOpacity[ind]);
  }
</script>

<div bind:this={root} class="root"
  style="
  --backgroundColor: {$$props.backgroundColor};
  --borderColor: {$$props.borderColor || $$props.color};
  --color: {$$props.color};
  --width: {$$props.width};
  --fontSize: {$$props.fontSize};
  --labelTop: {labelTop[+!!value]};
  --labelLeft: {labelLeft[+!!value]};
  --labelSize: {labelSize[+!!value]};
  --labelPadding: {labelPadding[+!!value]};
  --labelOpacity: {labelOpacity[+!!value]};
  "
  >
  <div class="decor" on:mouseleave={onFocus} on:mouseenter={onFocus}>
    <label bind:this={label} for="input">{title}</label>
    {#if $$props.select}
      <select bind:this={select} class="input" bind:value={value}>
        <option selected value></option>
        {#each $$props?.options as option}
          <option class="input" value={option}>{option[$$props?.option_key]}</option>
        {/each}
      </select>
    {:else if $$props.lines}
      <textarea bind:this={textarea} rows={$$props.lines} bind:value={value}/>
    {:else}
      <input bind:this={input} class="input" bind:value={value}>
    {/if}
  </div>
</div>

<style>
  .root {
    width: fit-content;
    height: fit-content;
    padding: 0.3em 2px 2px;
    overflow: hidden;
    color: var(--color);
    font-size: var(--fontSize);
  }
  .decor {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-content: center;
    width: var(--width);
    border: 2px solid var(--borderColor);
    background-color: var(--backgroundColor);
    font-size: var(--fontSize);
    border-radius: 1ch;
    position: relative;
    padding: 0.3em;
  }
  input, select, option, textarea {
    all: unset;
    width: 100%;
    line-height: 1em;
    overflow: visible;
    background-color: var(--backgroundColor);
    box-sizing: border-box;
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
  }
  select {
    padding-top: 0.07em;
    padding-bottom: 0.07em;
  }
  label {
    all: unset;
    width: fit-content;
    white-space: nowrap;
    line-height: 1em;
    background-color: var(--backgroundColor);
    border-radius: 0.25em;
    position: absolute;
    top: var(--labelTop);
    left: var(--labelLeft);
    padding: var(--labelPadding);
    opacity: var(--labelOpacity);
    font-size: var(--labelSize);

    transition-property: top, left, padding, opacity, font-size;
    transition-duration: 150ms;
    transition-timing-function: linear;
  }

</style>