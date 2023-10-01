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

{#if editing}
	<form on:submit|preventDefault={endEditing}>
		<input use:initFocus class="w-full bg-inherit" type="number" bind:value on:blur={endEditing} />
	</form>
{:else}
	<span on:click={startEditing}>
		{value === null ? '-' : value}
	</span>
{/if}
