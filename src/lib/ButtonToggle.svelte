<script lang="ts">

  export let titles = ["START", "STOP"];
  export let state = false;
  export let onToggle: (event: Event) => void;

  let title = titles[0];
  let button: HTMLButtonElement;

  $: state = (() => {
    state ? button?.classList.add('toggled') : button?.classList.remove('toggled')
    title = titles[state ? 1 : 0]
    return state;
  })();
</script>

<button bind:this={button} class="button {state ? 'toggled' : ''}" on:click={onToggle}>
  {title}
</button>

<style>
  .button {
    width: 9em;
    padding: 7px;
    box-shadow: 0px 5px 10px black;
    margin: 0px 12px 0px 0px;
    transition: box-shadow 250ms;
  }
  .toggled {
    box-shadow: 0px 2px 2px black;
    margin: 0px 12px 0px 0px;
  }
</style>