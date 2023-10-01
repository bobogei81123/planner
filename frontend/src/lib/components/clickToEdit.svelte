<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

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
	<form on:submit|preventDefault={endEditing}>
		<input class="w-full bg-inherit" type="text" bind:value on:blur={endEditing} use:initFocus/>
	</form>
{:else}
	<span on:click={startEditing}>
		{value}
	</span>
{/if}
