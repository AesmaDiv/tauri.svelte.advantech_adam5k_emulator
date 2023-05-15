<script lang="ts">
  export let state : boolean;
  export let channel : number;
  export let onChange : (channel: number, value: boolean) => void;

  let thumb : HTMLDivElement;
  let label : HTMLDivElement;

  function onChangeValue () {
    label?.style.setProperty("outline", "1px solid black");
    !!onChange && onChange(channel, thumb.classList.contains("enabled"));
  }

  $: current_state = (() => {
    label?.style.removeProperty("outline");
    return state;
  })();
</script>

<div class="channel">
  <div class="channel-index">{channel} :</div>
  <div bind:this={label}>{current_state ? 'ON' : 'OFF'}</div>
  <div class="switch-container" on:mousedown={onChangeValue}>
    <div bind:this={thumb} class="switch-thumb {current_state ? 'enabled' : ''}"/>
  </div>
</div>

<style>
  .channel {
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
    background-color: green;
    border-radius: 0.7em;
    display: flex;
    align-items: center;
  }
  .switch-thumb {
    width: 1.5em;
    height: 20px;
    background-color: white;
    border-radius: 0.65em;
    margin-left: 0.05em;
    transition: margin-left 250ms;
  }
  .switch-thumb.enabled {
    margin-left: 1.45em;
  }
</style>