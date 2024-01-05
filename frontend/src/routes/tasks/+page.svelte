<script lang="ts">
  import { getContextClient, mutationStore, queryStore } from '@urql/svelte';
  import { ArrowLeftOutline, ArrowRightOutline, PlusSolid } from 'flowbite-svelte-icons';
  import { Button, Select } from 'flowbite-svelte';
  import { addWeeks, formatISO, startOfWeek } from 'date-fns';
  import log from 'loglevel';

  import { checkNonNull } from '$lib/type_helpers';
  import TaskCard from '$lib/components/taskCard.svelte';
  import { graphql } from '$src/gql';
  import type { CreateTaskInput, Task } from '$src/gql/graphql';
  import { CREATE_TASK } from '$lib/task';

  let client = getContextClient();

  const viewOptions = [
    { value: 'all', name: 'All Tasks' },
    { value: 'weekly', name: 'Weekly Tasks' }
  ];
  type AllViews = 'all' | 'weekly';
  let selectedView: AllViews = 'weekly';

  class Week {
    #firstDateOfWeek: Date;

    constructor(firstDateOfWeek: Date) {
      this.#firstDateOfWeek = firstDateOfWeek;
    }

    static ofDate(date: Date): Week {
      return new Week(startOfWeek(date));
    }

    startDate(): Date {
      return this.#firstDateOfWeek;
    }

    endDate(): Date {
      return addWeeks(this.#firstDateOfWeek, 1);
    }

    nextWeek(): Week {
      return Week.ofDate(addWeeks(this.#firstDateOfWeek, 1));
    }

    prevWeek(): Week {
      return Week.ofDate(addWeeks(this.#firstDateOfWeek, -1));
    }
  }
  let selectedWeek: Week | null;
  $: {
    if (selectedView == 'all') {
      selectedWeek = null;
    } else {
      selectedWeek = Week.ofDate(new Date());
    }
  }
  function goToNextWeek() {
    if (selectedWeek == null) {
      log.warn('`selectedWeek` is null but `advanceWeek` is called. This should not happen');
      return;
    }
    selectedWeek = selectedWeek.nextWeek();
  }
  function goToPrevWeek() {
    if (selectedWeek == null) {
      log.warn('`selectedWeek` is null but `advanceWeek` is called. This should not happen');
      return;
    }
    selectedWeek = selectedWeek.prevWeek();
  }

  function formatISODate(date: Date): string {
    return formatISO(date, { representation: 'date' });
  }

  function getTaskFilter(selectedView: AllViews, selectedWeek: Week | null) {
    if (selectedView == 'all') {
      return null;
    }

    return {
      plannedDateRange: {
        start: formatISODate(selectedWeek!.startDate()),
        end: formatISODate(selectedWeek!.endDate())
      }
    };
  }

  $: allTasksStore = queryStore({
    client: getContextClient(),
    query: graphql(`
      query allTasks($filter: TaskFilter) {
        tasks(filter: $filter) {
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
    `),
    variables: {
      filter: getTaskFilter(selectedView, selectedWeek)
    }
  });

  let createTaskTitle = '';
  function createTask() {
    const input: CreateTaskInput = { title: createTaskTitle };
    if (selectedWeek != null) {
      input.plannedOn = formatISODate(selectedWeek.startDate());
    }
    mutationStore({
      client,
      query: CREATE_TASK,
      variables: { input }
    });
    createTaskTitle = '';
  }

  function sortByTaskId(t1: { id: any }, t2: { id: any }): number {
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
  <div class="flex justify-center flex-col mb-5">
    <Select class="mt-2" items={viewOptions} bind:value={selectedView} />
    {#if selectedView == 'weekly'}
      {@const week = checkNonNull(selectedWeek)}
      <div class="flex items-center justify-center h-12">
        <Button pill={true} class="!p-2" on:click={goToPrevWeek}>
          <ArrowLeftOutline class="w-3 h-3" />
        </Button>
        <span class="text-xl mx-3">
          {formatISODate(week.startDate())} – {formatISODate(week.endDate())}
        </span>
        <Button pill={true} class="!p-2" on:click={goToNextWeek}>
          <ArrowRightOutline class="w-3 h-3" />
        </Button>
      </div>
    {/if}
  </div>
  <div class="flex items-center h-16 bg-white mb-5">
    <div class="relative w-16 flex justify-center items-center">
      <PlusSolid />
    </div>
    <form class="w-full h-full mr-5" on:submit|preventDefault={createTask}>
      <input class="w-full h-full px-3" type="text" bind:value={createTaskTitle} />
    </form>
  </div>
  {#if $allTasksStore.fetching}
    <p>Loading...</p>
  {:else if $allTasksStore.error}
    <p>On no... {$allTasksStore.error.message}</p>
  {:else}
    {@const tasks = checkNonNull($allTasksStore.data).tasks.toSorted(sortByTaskId)}
    {#each tasks as task, i (task.id)}
      <div class="bg-white" class:border-b-2={i != tasks.length - 1}>
        <TaskCard {task} />
      </div>
    {/each}
  {/if}
</div>
