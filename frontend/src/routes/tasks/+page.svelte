<script lang="ts">
	import { graphql, type TaskStatus$options } from '$houdini';
	import TaskCard from '$lib/components/taskCard.svelte';
	import type { PageData } from './$houdini';
	import { PlusSolid } from 'flowbite-svelte-icons';
	export let data: PageData;

	const CREATE_TASK = graphql(`
		mutation CreateTask($title: String!) {
			createTask(input: {title: $title}) {
				id
        title
        status
				point
        ...CurrentTasks_insert
			}
		}
	`);
  let createTaskTitle = '';
  function createTask() {
    CREATE_TASK.mutate({ title: createTaskTitle });
    createTaskTitle = '';
  }

	$: ({ allTasks } = data);
	$: tasks = $allTasks.data!.tasks.toSorted((t1, t2) => {
		if (t1.id > t2.id) {
			return 1;
		} else if (t1.id < t2.id) {
			return -1;
		} else {
			return 0;
		}
	});
</script>

<div class="w-full min-h-screen flex justify-center bg-gray-200">
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
        <TaskCard {task}/>
			</div>
		{/each}
	</div>
</div>
