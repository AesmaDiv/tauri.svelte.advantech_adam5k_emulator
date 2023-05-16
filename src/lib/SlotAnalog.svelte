<script lang="ts">
  import { ADAMDATA, setAnalog } from "../shared/store";
  import ChannelAnalog from "./ChannelAnalog.svelte";

  export let slot_num : number = 0;

  const RANGES: string[] = ["8-bit","12-bit","15-bit","16-bit"];
  const MAXRNG: number[] = [   0xff,   0xfff,  0x7fff,  0xffff];

  let range : HTMLInputElement;

  let max_value = MAXRNG[3];
  let channels : Uint16Array = new Uint16Array(8)
  let cur_channel : number = 0;
  let cur_value : number = channels[cur_channel];

  function onChangeValue() {
    setAnalog(slot_num, cur_channel, range.valueAsNumber);
  }
  function onChangeText(event: Event) {
    range.valueAsNumber = (<HTMLInputElement>event.target).valueAsNumber;
    onChangeValue()
  }
  function onChangeChannel(channel: number) {
    cur_channel = channel;
    cur_value = channels[channel];
  }
  function onChangeRange(event: Event) {
    let select : HTMLSelectElement = <HTMLSelectElement>event.target;
    let option : number = select.options.selectedIndex;
    max_value = MAXRNG[option];
    if (cur_value > max_value) { cur_value = max_value };
  }

  ADAMDATA.subscribe(data => {
    let address = slot_num * 8;
    channels = data.analog.slice(address, address + 8);
  });
</script>

<div class="root">
  <input type="range" bind:this={range} max={max_value} bind:value={cur_value} on:change={onChangeValue}/>
  <div class="config">
    <input type="number" value={cur_value} on:change={onChangeText}/>
    <select on:change={onChangeRange}>
      {#each RANGES as rng}
      <option selected={rng === RANGES[3]}>{rng}</option>
      {/each}
    </select>
  </div>
  <div class="channels">
    {#each channels as val, i}
    <ChannelAnalog channel={i} value={val} selected={i === 0} onSelect={onChangeChannel}/>
    {/each}
  </div>
</div>

<style>
  .root {
    width: 130px;
    height: 290px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .config {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    gap: 5px;
  }
  .channels {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 5px;
  }

  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  input[type=range] {
    all: unset;
    width: 100%;
  }
  input[type="range"]::-webkit-slider-runnable-track {
   -webkit-appearance: none; /* Override default look */
   appearance: none;
   background-color: #3d3d3d;
   border-radius: 0.2em;
   z-index: 3;
  }
  input[type="range"]::-webkit-slider-thumb {
   -webkit-appearance: none; /* Override default look */
   appearance: none;
   background-color: #9b9b9b;
   width: 1em;
   height: 1em;
   border-radius: 0.2em;
   z-index: 3;
  }
  input[type=number] {
    width: 5ch;
    padding: 3px 1ch;
    font-size: medium;
  }
</style>