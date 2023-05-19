<script lang="ts">
  import { switchServerState, refreshRegisters, IPAddress } from "./shared/store";
  import SlotAnalog from "./lib/SlotAnalog.svelte";
  import SlotDigital from "./lib/SlotDigital.svelte";
  import Slider from "./lib/Slider.svelte";
  import TextBox from "./lib/TextBox.svelte";
  import ButtonToggle from "./lib/ButtonToggle.svelte";

  const BTN_TITLE = ['START SERVER', 'STOP SERVER'];
  let group_analog: string = 'SLOT 0';
  let group_digital: string = 'SLOT 0';
  let reader: NodeJS.Timer;
  let btn_state = false;

  async function onClick(event: Event) {
    switchServerState().then(state => {
      btn_state = state;
      if (state && !reader) {
        reader = setInterval(() => (async() => await refreshRegisters())(), 1000);
      } else {
        clearInterval(reader);
        reader = undefined;
      }
    });
  };

</script>

<main class="container">
  <div class="title">
    <TextBox title="IP ADDRESS" width="190px" backgroundColor="lightgrey" bind:value={$IPAddress}/>
    <ButtonToggle titles={BTN_TITLE} state={btn_state} onToggle={onClick}/>
  </div>
  <div class="slots_group">
    {#each Array(8) as _, i}
    <Slider header="SLOT {i}" bind:group={group_analog}>
      <SlotAnalog slotnum={i}/>
    </Slider>
    {/each}
  </div>
  <div class="slots_group">
    {#each Array(8) as _, i}
    <Slider header="SLOT {i}" bind:group={group_digital}>
      <SlotDigital slotnum={i}/>
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