<script lang="ts">
  import { cn } from '../utils';
  import { Button } from './ui/button';
  import * as Popover from './ui/popover';
  import type { CalendarDate } from '@internationalized/date';
  import { CalendarIcon } from 'lucide-svelte';
  import { Calendar } from './ui/calendar';

  export let value: CalendarDate | undefined;
</script>

<Popover.Root>
  <Popover.Trigger asChild let:builder>
    <Button
      variant="outline"
      class={cn(
        'w-full justify-start text-left font-normal',
        value == null && 'text-muted-foreground'
      )}
      builders={[builder]}
    >
      <CalendarIcon class="mr-2 h-4 w-4" />
      {value != null ? value : 'Pick a date'}
    </Button>
  </Popover.Trigger>
  <Popover.Content class="w-auto p-0">
    <Calendar bind:value {...$$restProps} />
  </Popover.Content>
</Popover.Root>
