<script lang="ts">
  import { getContextClient, mutationStore } from '@urql/svelte';
  import { parseDate, type DateValue } from '@internationalized/date';

  import { graphql } from '$src/gql';
  import { type Task } from '$src/gql/graphql';
  import { Button } from '$lib/components/ui/button';
  import * as Select from '$lib/components/ui/select';
  import ClickToEdit from '$lib/components/taskCard/clickToEdit.svelte';
  import ClickToEditNumber from '$lib/components/taskCard/clickToEditNumber.svelte';
  import { ChevronsUpDown, Trash2 } from 'lucide-svelte';
  import CircleCheckButton from './taskCard/circleCheckButton.svelte';
  import { Collapsible } from 'bits-ui';
  import DateSelector from './dateSelector.svelte';
  import { Label } from './ui/label';

  type InputTask = {
    id: Task['id'];
    title: Task['title'];
    scheduleDate?: Task['plannedOn'];
    cost: number | null;
  };
  export let task: InputTask;

  // This stops the strange "back propogation" of reactivity in Svelte.
  // See: https://github.com/sveltejs/svelte/issues/4933
  // `_task`, instead of `task`, should be used below.
  let _task: InputTask;
  function setTask(task: InputTask) {
    _task = task;
  }
  $: {
    setTask(task);
  }
  $: taskId = task.id;

  let client = getContextClient();

  // function invertedStatus(status: TaskStatus): TaskStatus {
  //   if (status === TaskStatus.Completed) {
  //     return TaskStatus.Active;
  //   } else {
  //     return TaskStatus.Completed;
  //   }
  // }
  // function toggleTaskStatus() {
  //   _task.status = invertedStatus(_task.status);
  //   mutationStore({
  //     client,
  //     query: graphql(`
  //       mutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {
  //         updateTask(input: { id: $id, status: $status }) {
  //           id
  //           status
  //         }
  //       }
  //     `),
  //     variables: { id: taskId, status: _task.status }
  //   });
  // }

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
      variables: { id: taskId, title }
    });
  }

  function updateTaskCost(cost: number | null) {
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskPoint($id: UUID!, $cost: Int) {
          updateTask(input: { id: $id, cost: $cost }) {
            id
            cost
          }
        }
      `),
      variables: { id: taskId, cost }
    });
  }

  $: scheduleDate = _task.scheduleDate != undefined ? parseDate(_task.scheduleDate) : undefined;
  function updateTaskScheduleDate(date: DateValue | undefined) {
    _task.scheduleDate = date?.toString();
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskScheduleDate($id: UUID!, $scheduleDate: NaiveDate) {
          updateTask(input: { id: $id, scheduleDate: $scheduleDate }) {
            id
            scheduleDate
          }
        }
      `),
      variables: { id: taskId, scheduleDate: _task.scheduleDate }
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
      variables: { id: taskId }
    });
  }
</script>

<div class="flex flex-col w-full rounded-lg border">
  <Collapsible.Root>
    <div class="flex items-center w-full h-20 relative group">
      <div class="ml-5 mr-3">
        <CircleCheckButton checked={false} />
      </div>
      <div class="font-sans flex-grow">
        <ClickToEdit
          bind:value={_task.title}
          on:changeSubmit={({ detail: newTitle }) => updateTaskTitle(newTitle)}
        />
      </div>
      <div class="mr-3 flex justify-center items-center">
        <div class="h-10 w-10 rounded bg-gray-500">
          <ClickToEditNumber
            bind:value={_task.cost}
            on:changeSubmit={({ detail: newPoint }) => updateTaskCost(newPoint)}
          />
        </div>
      </div>
      <Collapsible.Trigger class="mr-5">
        <ChevronsUpDown class="sq-4" />
      </Collapsible.Trigger>
    </div>

    <Collapsible.Content>
      <div class="px-4 py-2 grid grid-cols-2 gap-6">
        <div>
          <Label class="text-md" for={`${taskId}-date-selector`}><b>Schedule Date</b></Label>
          <DateSelector
            bind:value={scheduleDate}
            onValueChange={updateTaskScheduleDate}
            calendarLabel={`${taskId}-date-selector`}
            initialFocus
          />
        </div>
        <div>
        </div>
      </div>
      <div class="flex justify-end px-4 py-2 mb-2">
        <div class="w-30">
          <Button variant="destructive" class="mr-4 w-full" on:click={deleteTask}>
            <Trash2 class="mr-2 h-4 w-4" />Delete
          </Button>
        </div>
      </div>
    </Collapsible.Content>
  </Collapsible.Root>
</div>
