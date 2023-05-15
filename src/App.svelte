<script lang="ts">
  import { runServer, stopServer, refreshRegisters, printMap, IPAddress } from "./shared/store";
  import SlotAnalog from "./lib/SlotAnalog.svelte";
  import SlotDigital from "./lib/SlotDigital.svelte";
  import Slider from "./lib/Slider.svelte";
  import TextBox from "./lib/TextBox.svelte";

  let group_analog: string = 'SLOT 0';
  let group_digital: string = 'SLOT 0';
  let timer: NodeJS.Timer;
  let timer_title: string = 'RUN SERVER';

  async function onClick(event: Event) {
    switch ((<HTMLElement>event.target).id) {
      case "btn_read": {
        refresh();
      } break;
      case "btn_print": {
        runServer();
        // await printMap();
      } break;
    }
  }

  function refresh() {
    if (!timer) {
      timer = setInterval(() => (async() => await refreshRegisters())(), 1000);
      timer_title = 'STOP SERVER';
    } else {
      clearInterval(timer);
      timer = undefined;
      timer_title = 'RUN SERVER';
    }
  }
</script>

<main class="container">
  <div class="title">
    <TextBox title="IP:port" bind:value={$IPAddress}/>
    <div class="buttons">
      <button id="btn_read" class="button" on:click={onClick}>{timer_title}</button>
      <button id="btn_print" class="button" on:click={onClick}>&#62;_</button>
    </div>
  </div>
  <div class="slots_group">
    {#each Array(8) as _, i}
    <Slider header="SLOT {i}" bind:group={group_analog}>
      <SlotAnalog slot_num={i}/>
    </Slider>
    {/each}
  </div>
  <div class="slots_group">
    {#each Array(8) as _, i}
    <Slider header="SLOT {i}" bind:group={group_digital}>
      <SlotDigital slot_num={i}/>
    </Slider>
    {/each}
  </div>
</main>

<style>
  .container {
    font-size: 1em;
    display: flex;
    width: 100%;
    flex-direction: column;
    overflow: hidden;
    padding: 3px 0px;
  }
  .title {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
  .slots_group {
    display: flex;
    flex-direction: row;
  }
</style>