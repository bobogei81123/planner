<script lang="ts">
  import { getContextClient, mutationStore, queryStore } from '@urql/svelte';
  import log from 'loglevel';

  import { checkNonNull } from '$lib/type_helpers';
  import TaskCard from '$lib/components/taskCard.svelte';
  import { graphql } from '$src/gql';
  import type { AllTasksQuery, CreateTaskInput } from '$src/gql/graphql';
  import { CREATE_TASK } from '$lib/task';
  import * as Tabs from '$src/lib/components/ui/tabs';
  import { Button } from '$src/lib/components/ui/button';
  import { ChevronLeft, ChevronRight, Plus } from 'lucide-svelte';
  import Separator from '$src/lib/components/ui/separator/separator.svelte';
  import Input from '$src/lib/components/ui/input/input.svelte';
  import { Week, formatISODate } from '$lib/datetime';

  let client = getContextClient();

  type AllViews = 'all' | 'weekly';
  let selectedView: AllViews = 'weekly';
  let selectedWeek: Week | null = Week.ofDate(new Date());
  function goToNextWeek() {
    if (selectedWeek == null) {
      log.warn('bug: `selectedWeek` is null but `advanceWeek` is called.');
      return;
    }
    selectedWeek = selectedWeek.nextWeek();
  }
  function goToPrevWeek() {
    if (selectedWeek == null) {
      log.warn('bug: `selectedWeek` is null but `advanceWeek` is called.');
      return;
    }
    selectedWeek = selectedWeek.prevWeek();
  }

  function getTaskFilter(selectedView: AllViews, selectedWeek: Week | null) {
    if (selectedView == 'all') {
      return null;
    }

    return {
      scheduleDateRange: {
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
          scheduleDate
          title
          cost
        }
      }
    `),
    variables: {
      filter: getTaskFilter(selectedView, selectedWeek)
    }
  });
  let tasks: AllTasksQuery['tasks'];
  $: {
    if (!$allTasksStore.error && !$allTasksStore.fetching && !$allTasksStore.stale) {
      tasks = $allTasksStore.data!.tasks;
    }
  }

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

<div class="flex flex-col mt-5 w-2/5">
  <div class="flex justify-center flex-col mb-5">
    <Tabs.Root bind:value={selectedView}>
      <Tabs.List class="w-full grid grid-cols-3">
        <Tabs.Trigger value="all">All Tasks</Tabs.Trigger>
        <Tabs.Trigger value="weekly">Weekly Tasks</Tabs.Trigger>
        <Tabs.Trigger value="iterations">Iterations</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="weekly">
        {#if selectedView == 'weekly'}
          {@const week = checkNonNull(selectedWeek)}
          <div class="flex items-center justify-center h-12">
            <Button variant="outline" size="icon" on:click={goToPrevWeek}>
              <ChevronLeft class="w-5 h-5" />
            </Button>
            <span class="text-xl mx-3">
              {formatISODate(week.startDate())} – {formatISODate(week.lastDate())}
            </span>
            <Button variant="outline" size="icon" on:click={goToNextWeek}>
              <ChevronRight class="w-5 h-5" />
            </Button>
          </div>
        {/if}
      </Tabs.Content>
    </Tabs.Root>
  </div>
  <Separator />
  <div class="mt-5">
    <div class="flex items-center h-16 mb-5">
      <form class="w-full h-full mr-5" on:submit|preventDefault={createTask}>
        <Input
          class="w-full h-full px-3 text-lg focus-visible:border-input"
          type="text"
          bind:value={createTaskTitle}
          placeholder="Enter task title to create a new task"
        />
      </form>
      <Button class="h-full w-15" on:click={createTask}>
        <Plus class="w-7 h-7" />
      </Button>
    </div>
    {#if $allTasksStore.fetching}
      <p>Loading...</p>
    {:else if $allTasksStore.error}
      <p>On no... {$allTasksStore.error.message}</p>
    {:else}
      {#each tasks as task, i (task.id)}
        <div class:mt-3={i > 0}>
          <TaskCard {task}/>
        </div>
      {/each}
    {/if}
  </div>
</div>
