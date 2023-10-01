<script lang="ts">
	import { graphql, type TaskStatus$options } from '$houdini';
	import ClickToEdit from '$lib/components/clickToEdit.svelte';
	import ClickToEditNumber from '$lib/components/clickToEditNumber.svelte';
	import { AngleDownSolid, AngleUpSolid, TrashBinOutline } from 'flowbite-svelte-icons';
	import { Button } from 'flowbite-svelte';

	type Task = {
		readonly id: string;
		readonly title: string;
		readonly status: TaskStatus$options;
		readonly point: number | null;
	};
	export let task: Task;

	function statusIsCompleted(status: string): boolean {
		return status === 'COMPLETED';
	}

	function invertedStatus(status: TaskStatus$options): TaskStatus$options {
		if (statusIsCompleted(status)) {
			return 'ACTIVE';
		} else {
			return 'COMPLETED';
		}
	}

	const UPDATE_TASK_STATUS = graphql(`
		mutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {
			updateTask(input: { id: $id, status: $status }) {
				id
				status
			}
		}
	`);
	function updateTaskStatus(id: string, status: TaskStatus$options) {
		UPDATE_TASK_STATUS.mutate({ id, status });
	}

	const UPDATE_TASK_TITLE = graphql(`
		mutation UpdateTaskTitle($id: UUID!, $title: String) {
			updateTask(input: { id: $id, title: $title }) {
				id
				title
			}
		}
	`);
	function updateTaskTitle(id: string, title: string) {
		UPDATE_TASK_TITLE.mutate({ id, title });
	}

	const UPDATE_TASK_POINT = graphql(`
		mutation UpdateTaskPoint($id: UUID!, $point: Int) {
			updateTask(input: { id: $id, point: $point }) {
				id
				point
			}
		}
	`);
	function updateTaskPoint(id: string, point: number | null) {
		UPDATE_TASK_POINT.mutate({ id, point });
	}

	const DELETE_TASK = graphql(`
		mutation DeleteTask($id: UUID!) {
			deleteTask(id: $id) @Task_delete
		}
	`);
	function deleteTask() {
    console.log('delete')
		DELETE_TASK.mutate({ id: task.id });
	}

	let expanded = false;
</script>

<div class="flex flex-col w-full">
	<div class="flex items-center w-full h-20 relative group">
		<div class="relative w-20 flex justify-center items-center">
			<input
				type="checkbox"
				class="appearance-none w-10 h-10 border rounded-full checked:bg-green-100 focus:outline-2 focus:outline-green-600 hover:bg-green-100 peer"
				checked={statusIsCompleted(task.status)}
				on:click|preventDefault={() => updateTaskStatus(task.id, invertedStatus(task.status))}
			/>
			<div
				class="absolute w-6 h-6 pointer-events-none"
				class:hidden={!statusIsCompleted(task.status)}
			>
				<svg class="w-full h-full" viewBox="0 0 17.837 17.837">
					<path
						class="fill-green-600"
						d="M16.145,2.571c-0.272-0.273-0.718-0.273-0.99,0L6.92,10.804l-4.241-4.27
              c-0.272-0.274-0.715-0.274-0.989,0L0.204,8.019c-0.272,0.271-0.272,0.717,0,0.99l6.217,6.258c0.272,0.271,0.715,0.271,0.99,0
              L17.63,5.047c0.276-0.273,0.276-0.72,0-0.994L16.145,2.571z"
					/>
				</svg>
			</div>
		</div>
		<span class="font-sans text-lg flex-grow">
			<ClickToEdit
				bind:value={task.title}
				on:changeSubmit={({ detail: newTitle }) => updateTaskTitle(task.id, newTitle)}
			/>
		</span>
		<div class="w-20 flex justify-center items-center">
			<div class="h-10 w-10 rounded bg-gray-600 flex justify-center items-center">
				<span class="text-2xl text-white">
					<ClickToEditNumber
						bind:value={task.point}
						on:changeSubmit={({ detail: newPoint }) => updateTaskPoint(task.id, newPoint)}
					/>
				</span>
			</div>
		</div>
		<div class="absolute w-20 h-20 peer" style="right: -5rem;" />
		<div
			class="absolute w-12 h-12 bg-gray-50 flex justify-center items-center invisible group-hover:visible peer-hover:visible"
			style="right: -4.25rem;"
		>
			<button on:click={() => (expanded = !expanded)}>
				{#if !expanded}
					<AngleDownSolid />
				{:else}
					<AngleUpSolid />
				{/if}
			</button>
		</div>
	</div>

	{#if expanded}
		<div class="flex items-center justify-end w-full h-20">
			<Button color="red" class="mr-4" on:click={() => deleteTask()}><TrashBinOutline class="mr-2" />Delete</Button>
		</div>
	{/if}
</div>
