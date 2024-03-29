<script lang="ts">
  import ClickToEdit from '$lib/components/clickToEdit.svelte';
  import ClickToEditNumber from '$lib/components/clickToEditNumber.svelte';
  import { AngleDownSolid, AngleUpSolid, TrashBinOutline } from 'flowbite-svelte-icons';
  import { Button, Input, Label, Select } from 'flowbite-svelte';
  import { getContextClient, mutationStore } from '@urql/svelte';

  import { graphql } from '$src/gql';
  import { TaskStatus, type Task } from '$src/gql/graphql';

  type InputTask = {
    id: Task['id'];
    title: Task['title'];
    status: Task['status'];
    plannedOn?: Task['plannedOn'];
    point: number | null;
    iterations: { id: string }[];
  };
  type InputIteration = {
    id: string;
    name: string;
  };
  export let task: InputTask;
  export let allIterations: InputIteration[];

  let client = getContextClient();

  function invertedStatus(status: TaskStatus): TaskStatus {
    if (status === TaskStatus.Completed) {
      return TaskStatus.Active;
    } else {
      return TaskStatus.Completed;
    }
  }

  function updateTaskStatus(status: TaskStatus) {
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {
          updateTask(input: { id: $id, status: $status }) {
            id
            status
          }
        }
      `),
      variables: { id: task.id, status }
    });
  }
  function updateTaskTitle(title: string) {
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskTitle($id: UUID!, $title: String) {
          updateTask(input: { id: $id, title: $title }) {
            id
            title
          }
        }
      `),
      variables: { id: task.id, title }
    });
  }

  function updateTaskPoint(point: number | null) {
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskPoint($id: UUID!, $point: Int) {
          updateTask(input: { id: $id, point: $point }) {
            id
            point
          }
        }
      `),
      variables: { id: task.id, point }
    });
  }

  function updateTaskPlannedOn() {
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskPlannedOn($id: UUID!, $plannedOn: NaiveDate) {
          updateTask(input: { id: $id, plannedOn: $plannedOn }) {
            id
            point
          }
        }
      `),
      variables: { id: task.id, plannedOn: task.plannedOn }
    });
    console.log(task);
  }

  function updateTaskIteration(iteration: string | null) {
    console.log(iteration);
    const iterations = iteration === null ? [] : [iteration];
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {
          updateTask(input: { id: $id, iterations: $iterations }) {
            id
            iterations {
              id
              name
            }
          }
        }
      `),
      variables: { id: task.id, iterations }
    });
  }

  function deleteTask() {
    mutationStore({
      client,
      query: graphql(`
        mutation DeleteTask($id: UUID!) {
          deleteTask(id: $id)
        }
      `),
      variables: { id: task.id },
      context: {
        additionalTypenames: ['Task']
      }
    });
  }

  let allSelectItems: { value: string | null; name: string }[];
  $: {
    allSelectItems = [
      { value: null, name: '<None>' },
      ...allIterations.map((iter) => {
        return { value: iter.id, name: iter.name };
      })
    ];
  }

  let selectedIterationShadow: string | null = null;
  function getSelectedIterationId(iterations: { id: string }[]): null | string {
    if (iterations.length == 0) {
      return null;
    }
    return iterations[0].id;
  }
  $: {
    selectedIterationShadow = getSelectedIterationId(task.iterations);
  }
  $: selectedIteration = selectedIterationShadow;

  let expanded = false;
</script>

<div class="flex flex-col w-full">
  <div class="flex items-center w-full h-20 relative group">
    <div class="relative w-20 flex justify-center items-center">
      <input
        type="checkbox"
        class="appearance-none w-10 h-10 border rounded-full checked:bg-green-100 focus:outline-2 focus:outline-green-600 hover:bg-green-100 peer"
        checked={task.status === TaskStatus.Completed}
        on:click|preventDefault={() => updateTaskStatus(invertedStatus(task.status))}
      />
      <div
        class="absolute w-6 h-6 pointer-events-none"
        class:hidden={task.status !== TaskStatus.Completed}
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
    <span class="font-sans flex-grow">
      <ClickToEdit
        bind:value={task.title}
        on:changeSubmit={({ detail: newTitle }) => updateTaskTitle(newTitle)}
      />
    </span>
    <div class="w-20 flex justify-center items-center">
      <div class="h-10 w-10 rounded bg-gray-600">
        <ClickToEditNumber
          bind:value={task.point}
          on:changeSubmit={({ detail: newPoint }) => updateTaskPoint(newPoint)}
        />
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
    <div class="px-4 py-2 grid grid-cols-2 gap-6">
      <div>
        <Label class="text-md">
          <b>Planned On</b>
          <Input type="date" bind:value={task.plannedOn} on:change={updateTaskPlannedOn} />
        </Label>
      </div>
      <div>
        <Label class="text-md">
          <b>Iteration</b>
          <Select
            items={allSelectItems}
            bind:value={selectedIteration}
            on:change={() => updateTaskIteration(selectedIteration)}
            class="w-full"
          />
        </Label>
      </div>
    </div>
    <div class="flex justify-end px-4 py-2 mb-2">
      <div class="w-30">
        <Button color="red" class="mr-4 w-full" on:click={deleteTask}>
          <TrashBinOutline class="mr-2" />Delete
        </Button>
      </div>
    </div>
  {/if}
</div>
