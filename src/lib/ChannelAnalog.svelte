<script lang="ts">
  export let channel : number;
  export let value : number;
  export let selected : boolean;
  export let onSelect : (channel: number) => void;

  const onChange = () => !!onSelect && onSelect(channel);
</script>


<div class="channel">
  <div class="channel-index">{channel} :</div>
  <div class="channel-value">{value}</div>
  <input type="radio" name="channel" id="channel {channel}" checked={selected} on:change={onChange}/>
</div>

<style>
    .channel {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    gap: 5px;
  }
  .channel-value {
    width: 5ch;
    outline: 1px solid black;
    border-radius: 3px;
    padding: 0px 1ch;
  }
  .channel-index {
    width: fit-content;
    color: blue;
    margin-left: 2px;
  }
  input[type=radio] {
    width: fit-content;
    margin-bottom: 2px;
  }
</style>