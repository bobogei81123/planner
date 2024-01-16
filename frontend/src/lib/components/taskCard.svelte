<script lang="ts">
  import { Label } from 'flowbite-svelte';
  import { getContextClient, mutationStore } from '@urql/svelte';
  import { parseDate, type DateValue } from '@internationalized/date';

  import { graphql } from '$src/gql';
  import { TaskStatus, type Task } from '$src/gql/graphql';
  import * as Popover from '$lib/components/ui/popover';
  import { Calendar } from '$lib/components/ui/calendar';
  import { Button } from '$lib/components/ui/button';
  import * as Select from '$lib/components/ui/select';
  import { cn } from '$lib/utils';
  import ClickToEdit from '$lib/components/taskCard/clickToEdit.svelte';
  import ClickToEditNumber from '$lib/components/taskCard/clickToEditNumber.svelte';
  import { CalendarIcon, ChevronsUpDown, Trash2 } from 'lucide-svelte';
  import CircleCheckButton from './taskCard/circleCheckButton.svelte';
  import { Collapsible } from 'bits-ui';

  type InputTask = {
    id: Task['id'];
    title: Task['title'];
    status: Task['status'];
    plannedOn?: Task['plannedOn'];
    point: number | null;
    iterations: { id: string; name: string }[];
  };
  type InputIteration = {
    id: string;
    name: string;
  };
  export let task: InputTask;
  export let allIterations: InputIteration[];

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

  function invertedStatus(status: TaskStatus): TaskStatus {
    if (status === TaskStatus.Completed) {
      return TaskStatus.Active;
    } else {
      return TaskStatus.Completed;
    }
  }
  function toggleTaskStatus() {
    _task.status = invertedStatus(_task.status);
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
      variables: { id: taskId, status: _task.status }
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
      variables: { id: taskId, title }
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
      variables: { id: taskId, point }
    });
  }

  $: plannedOnDate = _task.plannedOn != undefined ? parseDate(_task.plannedOn) : undefined;
  function updateTaskPlannedOn(date: DateValue | undefined) {
    _task.plannedOn = date?.toString();
    mutationStore({
      client,
      query: graphql(`
        mutation UpdateTaskPlannedOn($id: UUID!, $plannedOn: NaiveDate) {
          updateTask(input: { id: $id, plannedOn: $plannedOn }) {
            id
            plannedOn
          }
        }
      `),
      variables: { id: taskId, plannedOn: _task.plannedOn }
    });
  }

  interface UiSelectedIteration {
    value: string | null;
    label?: string;
  }
  function getUiSelectedIteration(iterations: InputIteration[]): UiSelectedIteration {
    if (iterations.length == 0) {
      return { value: null, label: '<None>' };
    }
    return { value: iterations[0].id, label: iterations[0].name };
  }
  $: uiSelectedIteration = getUiSelectedIteration(_task.iterations);
  function updateTaskIteration(uiIteration: UiSelectedIteration | undefined) {
    if (uiIteration == undefined) {
      return;
    }
    _task.iterations =
      uiIteration.value == null ? [] : [{ id: uiIteration.value, name: uiIteration.label! }];
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
      variables: { id: taskId, iterations: _task.iterations.map((it) => it.id) }
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
        <CircleCheckButton
          checked={_task.status === TaskStatus.Completed}
          on:click={toggleTaskStatus}
        />
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
            bind:value={_task.point}
            on:changeSubmit={({ detail: newPoint }) => updateTaskPoint(newPoint)}
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
          <Label class="text-md">
            <b>Planned On</b>
            <Popover.Root>
              <Popover.Trigger asChild let:builder>
                <Button
                  variant="outline"
                  class={cn(
                    'w-full justify-start text-left font-normal',
                    plannedOnDate == undefined && 'text-muted-foreground'
                  )}
                  builders={[builder]}
                >
                  <CalendarIcon class="mr-2 h-4 w-4" />
                  {plannedOnDate != undefined ? plannedOnDate : 'Pick a date'}
                </Button>
              </Popover.Trigger>
              <Popover.Content class="w-auto p-0">
                <Calendar
                  bind:value={plannedOnDate}
                  onValueChange={updateTaskPlannedOn}
                  initialFocus
                />
              </Popover.Content>
            </Popover.Root>
          </Label>
        </div>
        <div>
          <Label class="text-md">
            <b>Iteration</b>
            <Select.Root
              onSelectedChange={updateTaskIteration}
              bind:selected={uiSelectedIteration}
            >
              <Select.Trigger class="w-full">
                <Select.Value placeholder="Select an iteration" />
              </Select.Trigger>
              <Select.Content>
                <Select.Item value={null} label="<None>">{'<None>'}</Select.Item>
                {#each allIterations as item}
                  <Select.Item value={item.id} label={item.name}>{item.name}</Select.Item>
                {/each}
              </Select.Content>
            </Select.Root>
          </Label>
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
