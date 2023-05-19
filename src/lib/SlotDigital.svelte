<script lang="ts">
  import { ADAMDATA } from "../shared/store";
  import ChannelDigital from "./ChannelDigital.svelte";

  export let slotnum : number = 0;

  let channels : Uint8Array = new Uint8Array(8);

  ADAMDATA.subscribe(data => {
    let address = slotnum * 8;
    channels = data.digital.slice(address, address + 8);
  });
</script>

<div class="root">
  <div class="channels">
    {#each channels as val, channel}
    <ChannelDigital {slotnum} {channel} state={val === 1}/>
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
    height: 250px;
    display: flex;
    flex-flow: column wrap;
    gap: 5px;
  }

</style>