<script lang="ts">
  import { ADAMDATA, setDigital } from "../shared/store";
  import ChannelDigital from "./ChannelDigital.svelte";

  export let slot_num : number = 0;

  let channels : Uint8Array = new Uint8Array([0,1,0,1,0,1,0,1]);

  function onChangeValue(channel: number, value: boolean) {
    setDigital(slot_num, channel, value ? 0: 1);
  }

  ADAMDATA.subscribe(data => {
    let address = slot_num * 8;
    channels = data.digital.slice(address, address + 8);
    // state?.style.removeProperty("outline");
  });
</script>

<div class="root">
  <div class="channels">
    {#each channels as val, i}
      <ChannelDigital channel={i} state={val === 1} onChange={onChangeValue}/>
    {/each}
  </div>
</div>

<style>
  .root {
    width: 130px;
    height: 225px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: stretch;
    padding: 10px;
    gap: 10px;
    --state: 1px solid black;
  }
  .channels {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 5px;
  }

</style>