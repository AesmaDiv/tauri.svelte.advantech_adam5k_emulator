<script lang="ts">
  import { setDigital } from "../shared/store";
  export let state : boolean;
  export let slotnum : number;
  export let channel : number;

  let thumb : HTMLDivElement;
  let label : HTMLDivElement;

  function onChangeValue () {
    label?.style.setProperty("outline", "1px solid black");
    let value = thumb.classList.contains("enabled") ? 0 : 1;
    setDigital(slotnum, channel, value);
  }

  $: current_state = (() => {
    label?.style.removeProperty("outline");
    return state;
  })();
</script>

<div class="channel">
  <div class="channel-index">{channel} :</div>
  <div bind:this={label}>{current_state ? 'ON' : 'OFF'}</div>
  <div class="switch-container {current_state ? 'enabled' : ''}" on:mousedown={onChangeValue}>
    <div bind:this={thumb} class="switch-thumb {current_state ? 'enabled' : ''}"/>
  </div>
</div>

<style>
  .channel {
    /* width: fit-content; */
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    gap: 5px;
  }
  .channel-index {
    width: fit-content;
    color: blue;
    margin-left: 2px;
  }
  .switch-container {
    width: 3em;
    height: 1.4em;
    border-radius: 0.7em;
    display: flex;
    align-items: center;
    background-color: grey;
    transition: background-color 250ms;
  }
  .switch-container.enabled {
    background-color: green;
  }
  .switch-thumb {
    width: 1.3em;
    height: 1.3em;
    background-color: white;
    border-radius: 0.65em;
    margin-left: 0.05em;
    transition: margin-left 250ms;
  }
  .switch-thumb.enabled {
    margin-left: 1.65em;
    
  }
</style>