<script lang="ts">
  import { setAnalog } from "../shared/store";

  export let slotnum : number;
  export let channel : number;
  export let value : number;
  export let maximum : number;
  export let selected : boolean;
  export let onSelect : (channel: number) => void;

  let input : HTMLInputElement;

  const onRadio = () => !!onSelect && onSelect(channel);
  function onChangeValue() {
    let val = Math.min(maximum, input?.valueAsNumber || 0);
    input?.style.setProperty("outline", "2px solid black");
    setAnalog(slotnum, channel, val);
  }

  $: current_value = (() => {
    input?.style.removeProperty("outline");
    return value;
  })();
</script>


<div class="channel">
  <div class="channel-index">{channel} :</div>
  <!-- <div class="channel-value">{value}</div> -->
  <input type="number" value="{current_value}" on:change={onChangeValue} bind:this={input}/>
  <input type="radio" name="channel" id="channel {channel}" checked={selected} on:change={onRadio}/>
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
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  input[type=number] {
    width: 5ch;
    padding: 1px 1ch;
    font-size: medium;
  }
  input[type=radio] {
    width: fit-content;
    margin-bottom: 2px;
  }
</style>