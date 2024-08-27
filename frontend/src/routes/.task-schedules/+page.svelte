<script lang="ts">
  import { getContextClient, mutationStore, queryStore } from '@urql/svelte';
  import log from 'loglevel';
  import { today, type CalendarDate, getLocalTimeZone } from '@internationalized/date';

  import { ALL_WEEK_DAYS } from '$src/lib/datetime';
  import { graphql } from '$src/gql';
  import TaskScheduleCard from '$src/lib/components/taskScheduleCard.svelte';
  import Button from '$src/lib/components/ui/button/button.svelte';
  import { PlusIcon } from 'lucide-svelte';
  import * as Drawer from '$lib/components/ui/drawer';
  import { Label } from '$src/lib/components/ui/label';
  import { Input } from '$src/lib/components/ui/input';
  import ClickToEditNumber from '$src/lib/components/taskCard/clickToEditNumber.svelte';
  import * as Select from '$src/lib/components/ui/select';
  import { Checkbox } from '$src/lib/components/ui/checkbox';
  import type { CreateTaskScheduleInput } from '$src/gql/graphql';
  import { checkNonNull } from '$src/lib/type_helpers';
  import DateSelector from '$src/lib/components/dateSelector.svelte';

  let client = getContextClient();

  $: allTaskSchedulesStore = queryStore({
    client: getContextClient(),
    query: graphql(`
      query allTaskSchedules {
        taskSchedules {
          id
          nextDateToCheck
          dateSpec
          taskTitle
          taskPoint
        }
      }
    `)
  });

  function defaultRepeatedDays() {
    return Array.from({ length: 7 }, () => false);
  }

  let taskTitle: string = '';
  let taskPoint: number | null = null;
  let uiSelectedRepeatedType = { value: 'weekly', label: 'Weekly' };
  let startDate: CalendarDate | undefined = undefined;
  let repeatedDays = defaultRepeatedDays();

  function createTaskSchedule() {
    if (uiSelectedRepeatedType.value != 'weekly') {
      log.error(`Repeat type ${uiSelectedRepeatedType.value} is not supported.`);
      return;
    }
    if (startDate == null) {
      startDate = today(getLocalTimeZone());
    }

    const weekDays = [];
    for (let i = 0; i < repeatedDays.length; i++) {
      if (repeatedDays[i]) {
        weekDays.push(ALL_WEEK_DAYS[i]);
      }
    }
    const dateSpec = {
      RepeatsWeekly: {
        start_date: startDate.toString(),
        week_days: weekDays,
        every_n_week: 1
      }
    };
    const input: CreateTaskScheduleInput = {
      taskTitle,
      taskPoint,
      dateSpec
    };

    mutationStore({
      client,
      query: graphql(`
        mutation CreateTaskSchedule($input: CreateTaskScheduleInput!) {
          createTaskSchedule(input: $input) {
            id
            nextDateToCheck
            dateSpec
            taskTitle
            taskPoint
          }
        }
      `),
      variables: { input }
    });
    taskTitle = '';
    taskPoint = null;
    startDate = undefined;
    repeatedDays = defaultRepeatedDays();
  }
</script>

<div class="flex flex-col mt-5 w-2/5 space-y-5">
  <div class="mt-5 flex flex-col gap-2">
    {#if $allTaskSchedulesStore.fetching}
      <p>Loading...</p>
    {:else if $allTaskSchedulesStore.error}
      <p>On no... {$allTaskSchedulesStore.error.message}</p>
    {:else}
      {#each checkNonNull($allTaskSchedulesStore.data).taskSchedules as taskSchedule}
        <TaskScheduleCard {taskSchedule} />
      {/each}
    {/if}
  </div>
  <div class="flex justify-center">
    <Drawer.Root>
      <Drawer.Trigger asChild let:builder>
        <Button builders={[builder]}><PlusIcon class="h-4 w-4 mr-2" />Add Task</Button>
      </Drawer.Trigger>
      <Drawer.Content>
        <div class="mx-auto w-full max-w-sm">
          <Drawer.Header>
            <Drawer.Title>Add Task Schedule</Drawer.Title>
          </Drawer.Header>
          <div class="flex-col items-center justify-center mt-4 space-y-2">
            <div class="flex flex-col w-full gap-1.5 text-md">
              <Label for="task-title"><b>Task Title</b></Label>
              <div class="flex space-x-4">
                <div class="grow">
                  <Input bind:value={taskTitle} type="text" id="task-title" placeholder="Title" />
                </div>
                <div class="h-10 w-10 rounded bg-gray-500">
                  <ClickToEditNumber bind:value={taskPoint} />
                </div>
              </div>
            </div>
            <div>
              <Label class="text-md">
                <b>Repeat Pattern</b>
                <Select.Root bind:selected={uiSelectedRepeatedType}>
                  <Select.Trigger class="w-full">
                    <Select.Value placeholder="Select repeat pattern type" />
                  </Select.Trigger>
                  <Select.Content>
                    <Select.Item value="weekly" label="Weekly">Weekly</Select.Item>
                  </Select.Content>
                </Select.Root>
              </Label>
            </div>
            <div>
              <b>Start Date</b>
              <DateSelector bind:value={startDate} initialFocus />
            </div>
            <div>
              <b>Every</b>
              <div class="flex space-x-2">
                {#each ALL_WEEK_DAYS as day, i (i)}
                  <div class="flex items-center space-x-1">
                    <span>{day}</span>
                    <Checkbox bind:checked={repeatedDays[i]}>{day}</Checkbox>
                  </div>
                {/each}
              </div>
            </div>
          </div>
          <Drawer.Footer>
            <Button on:click={createTaskSchedule}>Submit</Button>
            <Drawer.Close asChild let:builder>
              <Button builders={[builder]} variant="outline">Cancel</Button>
            </Drawer.Close>
          </Drawer.Footer>
        </div>
      </Drawer.Content>
    </Drawer.Root>
  </div>
</div>
