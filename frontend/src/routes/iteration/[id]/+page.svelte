<script lang="ts">
  import { getContextClient, mutationStore, queryStore } from '@urql/svelte';
  import { PlusSolid } from 'flowbite-svelte-icons';

  import { checkNonNull } from '$lib/type_helpers';
  import TaskCard from '$lib/components/taskCard.svelte';
  import { graphql } from '$src/gql';
  import { CREATE_TASK } from '$src/lib/task';

  export let data: { id: string };

  let client = getContextClient();

  $: iterationStore = queryStore({
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
            iterations {
              id
              name
            }
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
      variables: { title: createTaskTitle, iteration: data.id },
      context: {
        additionalTypenames: ['Iteration']
      }
    });
  }

  function sortByTaskId(t1: { id: string }, t2: { id: string }): number {
    if (t1.id > t2.id) {
      return 1;
    } else if (t1.id < t2.id) {
      return -1;
    } else {
      return 0;
    }
  }
</script>

<div class="flex flex-col mt-5 w-1/3">
  {#if $iterationStore.fetching}
    <p>Loading...</p>
  {:else if $iterationStore.error}
    <p>On no... {$iterationStore.error.message}</p>
  {:else}
    {@const tasks = checkNonNull($iterationStore.data).iteration.tasks.toSorted(sortByTaskId)}
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
  {/if}
</div>
