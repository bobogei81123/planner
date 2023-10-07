<script lang="ts">
	import { getContextClient, mutationStore, queryStore } from '@urql/svelte';
	import { PlusSolid } from 'flowbite-svelte-icons';

	import { checkNonNull } from '$lib/type_helpers';
	import TaskCard from '$lib/components/taskCard.svelte';
	import { graphql } from '$src/gql';
	import type { Task } from '$src/gql/graphql';
	import { CREATE_TASK } from '$src/lib/task';

	export let data: { id: string };

	let client = getContextClient();

	const iterationStore = queryStore({
		client: getContextClient(),
		query: graphql(`
			query allTasksInIteration($id: UUID!) {
				iteration(id: $id) {
					id
					name
					tasks {
						id
						title
						status
						point
					}
				}
			}
		`),
		variables: { id: data.id }
	});

	let createTaskTitle = '';
	function createTask() {
		mutationStore({
			client,
			query: CREATE_TASK,
			variables: { title: createTaskTitle, planned_for: data.id }
		});
	}

	function sortByTaskId(t1: Task, t2: Task): number {
		if (t1.id > t2.id) {
			return 1;
		} else if (t1.id < t2.id) {
			return -1;
		} else {
			return 0;
		}
	}
</script>

<div class="w-full min-h-screen flex justify-center bg-gray-200">
	{#if $iterationStore.fetching}
		<p>Loading...</p>
	{:else if $iterationStore.error}
		<p>On no... {$iterationStore.error.message}</p>
	{:else}
		{@const tasks = checkNonNull($iterationStore.data).iteration.tasks.toSorted(sortByTaskId)}
		<div class="flex flex-col mt-5 w-1/3">
			<div class="flex items-center h-16 bg-white mb-5">
				<div class="relative w-16 flex justify-center items-center">
					<PlusSolid />
				</div>
				<form class="w-full h-full mr-5" on:submit|preventDefault={createTask}>
					<input class="w-full h-full px-3" type="text" bind:value={createTaskTitle} />
				</form>
			</div>
			{#each tasks as task, i (task.id)}
				<div class="bg-white" class:border-b-2={i != tasks.length - 1}>
					<TaskCard {task} />
				</div>
			{/each}
		</div>
	{/if}
</div>

<h1>
	{data.id}
</h1>
