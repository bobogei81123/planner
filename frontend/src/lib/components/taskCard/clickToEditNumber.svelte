<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  export let value: number | null;
  let originValue: number | null;
  let editing = false;
  const dispatch = createEventDispatcher();

  onMount(() => {
    originValue = value;
  });

  function startEditing() {
    editing = true;
  }

  function endEditing() {
    if (!editing) return;
    if (value !== originValue) {
      dispatch('changeSubmit', value);
    }
    originValue = value;
    editing = false;
  }

  function initFocus(el: HTMLElement) {
    el.focus();
  }
</script>

<div class="w-full h-full flex items-center justify-center text-white">
  {#if editing}
    <form on:submit|preventDefault={endEditing}>
      <input
        use:initFocus
        class="w-full bg-inherit text-2xl px-0 text-center"
        type="number"
        bind:value
        on:blur={endEditing}
      />
    </form>
  {:else}
    <span on:click={startEditing} class="text-2xl">
      {value === null ? '-' : value}
    </span>
  {/if}
</div>

<style>
  input[type='number'] {
    -moz-appearance: textfield;
    appearance: textfield;
  }

  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    appearance: none;
  }
</style>
