<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  import Input from '../ui/input/input.svelte';

  export let value: string;
  let originValue: string;
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

{#if editing}
  <div class="-translate-x-1">
    <form on:submit|preventDefault={endEditing}>
      <input
        class="w-full bg-inherit text-lg px-1 rounded-md border focus-visible:border-input focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background"
        type="text"
        bind:value
        on:blur={endEditing}
        use:initFocus
      />
    </form>
  </div>
{:else}
  <span on:click={startEditing} class="text-lg">
    {value}
  </span>
{/if}
