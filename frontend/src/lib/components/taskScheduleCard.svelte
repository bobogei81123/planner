<script lang="ts">
  import log from 'loglevel';

  import type { TaskSchedule } from '$src/gql/graphql';
  import * as Popover from '$lib/components/ui/popover';
  import { Button } from '$lib/components/ui/button';
  import * as Select from '$lib/components/ui/select';
  import { cn } from '$lib/utils';
  import ClickToEdit from '$lib/components/taskCard/clickToEdit.svelte';
  import ClickToEditNumber from '$lib/components/taskCard/clickToEditNumber.svelte';
  import { CalendarIcon, ChevronsUpDown } from 'lucide-svelte';
  import CircleCheckButton from './taskCard/circleCheckButton.svelte';
  import { Collapsible } from 'bits-ui';
  import { Label } from './ui/label';
  import { Checkbox } from './ui/checkbox';
  import { ALL_WEEK_DAYS } from '../datetime';

  type InputTaskSchedule = {
    id: TaskSchedule['id'];
    dateSpec: TaskSchedule['dateSpec'];
    taskTitle: TaskSchedule['taskTitle'];
    taskPoint: TaskSchedule['taskPoint'];
  };
  export let taskSchedule: InputTaskSchedule;

  // This stops the strange "back propogation" of reactivity in Svelte.
  let _taskSchedule: InputTaskSchedule;
  function setTaskSchedule(taskSchedule: InputTaskSchedule) {
    _taskSchedule = taskSchedule;
  }
  $: {
    setTaskSchedule(taskSchedule);
  }
  $: taskPoint = taskSchedule.taskPoint ?? null;

  interface UiSelectedRepeatedType {
    value: string | null;
    label?: string;
  }
  function getUiSelectedRepeatedType(dateSpec: any): UiSelectedRepeatedType {
    if ('RepeatsWeekly' in dateSpec) {
      return {
        value: 'weekly',
        label: 'Weekly'
      };
    }
    log.error(`Unknown repeated type ${JSON.stringify(dateSpec)}`);
    throw new Error('Unknown repeated type');
  }
  $: uiSelectedRepeatedType = getUiSelectedRepeatedType(_taskSchedule.dateSpec);
</script>

<div class="flex-col w-full rounded-lg border">
  <Collapsible.Root>
    <div class="flex items-center w-full h-20 relative group">
      <div class="ml-5 mr-3">
        <CircleCheckButton checked={false} />
      </div>
      <div class="font-sans flex-grow">
        <ClickToEdit bind:value={_taskSchedule.taskTitle} />
      </div>
      <div class="mr-3 flex justify-center items-center">
        <div class="h-10 w-10 rounded bg-gray-500">
          <ClickToEditNumber bind:value={taskPoint} />
        </div>
      </div>
      <Collapsible.Trigger class="mr-5">
        <ChevronsUpDown class="sq-4" />
      </Collapsible.Trigger>
    </div>

    <Collapsible.Content>
      <div class="flex-col">
        <div class="px-4 py-2 grid grid-cols-2 gap-6">
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
          <Label class="text-md">
            <b>Start Date</b>
            <Popover.Root>
              <Button
                variant="outline"
                class={cn(
                  'w-full justify-start text-left font-normal',
                  _taskSchedule.dateSpec.RepeatsWeekly.start_date == undefined &&
                    'text-muted-foreground'
                )}
              >
                <CalendarIcon class="mr-2 h-4 w-4" />
                {_taskSchedule.dateSpec.RepeatsWeekly.start_date != undefined
                  ? taskSchedule.dateSpec.RepeatsWeekly.start_date
                  : 'Pick a date'}
              </Button>
            </Popover.Root>
          </Label>
        </div>
        <div class="px-4 py-2 w-full flex items-center space-x-2">
          <b>Every</b>
          {#each ALL_WEEK_DAYS as day}
            {@const checked = _taskSchedule.dateSpec.RepeatsWeekly.week_days.includes(day)}
            <div class="flex items-center space-x-1">
              <span>{day}</span>
              <Checkbox {checked}>{day}</Checkbox>
            </div>
          {/each}
        </div>
      </div>
    </Collapsible.Content>
  </Collapsible.Root>
</div>
