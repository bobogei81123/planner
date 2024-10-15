import { useMutation, useQuery } from '@apollo/client';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useSearchParams } from 'react-router-dom';

import { CreateTaskDialog } from '@/components/EditTaskDialog';
import TaskList, { ListType } from '@/components/TaskList';
import { Button } from '@/components/ui/button.tsx';
import { Calendar } from '@/components/ui/calendar';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Separator } from '@/components/ui/separator.tsx';
import { CreateTaskInput, ViewType } from '@/graphql/generated/graphql';
import { CREATE_TASK, LIST_TASKS } from '@/graphql/task.ts';
import { Epoch, EpochTypeString, Task } from '@/lib/task';

function useEpoch() {
  const [searchParams, setSearchParams] = useSearchParams();

  function tryParseEpoch(epochParam: string | null): Epoch | undefined {
    if (epochParam == null) return undefined;
    return Epoch.fromUrlParam(epochParam);
  }

  let epoch = tryParseEpoch(searchParams.get('epoch'));
  if (epoch === undefined) {
    epoch = Epoch.ofDate(new Date());
    setSearchParams({ epoch: epoch.toUrlParam() });
  }
  const setEpoch = (epoch: Epoch) => {
    setSearchParams({ epoch: epoch.toUrlParam() });
  };
  return [epoch, setEpoch] as const;
}

export default function Tasks() {
  const [epoch, setEpoch] = useEpoch();
  const {
    loading: scheduledTasksLoading,
    error: scheduledTaskError,
    data: scheduledTask,
  } = useQuery(LIST_TASKS, {
    variables: {
      viewType: ViewType.Scheduled,
      epoch: epoch.toGQL(),
    },
  });
  const {
    loading: plannedTasksLoading,
    error: plannedTasksError,
    data: plannedTasks,
  } = useQuery(LIST_TASKS, {
    variables: {
      viewType: ViewType.Planned,
      epoch: epoch.toGQL(),
    },
  });
  const [createTaskMutation] = useMutation(CREATE_TASK, {
    refetchQueries: [LIST_TASKS],
  });

  function setEpochSelectType(type: EpochTypeString) {
    const currentDate = epoch.startDate() ?? new Date();
    switch (type) {
      case 'DATE':
        setEpoch(Epoch.ofDate(new Date()));
        break;
      case 'WEEK':
        setEpoch(Epoch.ofWeek(currentDate));
        break;
      case 'ALL':
        setEpoch(Epoch.nullEpoch());
        break;
    }
  }

  if (scheduledTasksLoading || plannedTasksLoading) return <p>Loading...</p>;
  if (scheduledTaskError || plannedTasksError) return <p>Error : {scheduledTaskError?.message}</p>;

  function onCreateTask(data: CreateTaskInput) {
    void createTaskMutation({
      variables: data,
    });
  }

  function setEpochDate(date?: Date) {
    if (date == undefined) return;
    switch (epoch.epochTypeString()) {
      case 'WEEK':
        setEpoch(Epoch.ofWeek(date));
        break;
      case 'DATE':
        setEpoch(Epoch.ofDate(date));
        break;
      case 'ALL':
        console.warn('bug: `setEpochDate` called when epoch type is `ALL`');
        return;
    }
  }

  let epochDisplayString;
  if (epoch.isNullEpoch()) {
    epochDisplayString = <span>{epoch.toDisplayString()}</span>;
  } else {
    epochDisplayString = (
      <Popover>
        <PopoverTrigger asChild>
          <span>{epoch.toDisplayString()}</span>
        </PopoverTrigger>
        <PopoverContent className="w-auto p-0">
          <Calendar
            mode="single"
            selected={epoch.startDate()}
            onSelect={setEpochDate}
            initialFocus
          />
        </PopoverContent>
      </Popover>
    );
  }

  return (
    <div className="flex justify-center">
      <div className="w-full max-w-screen-md h-screen flex-col">
        <div className="w-full my-4 flex flex-col items-center space-y-2">
          <Select value={epoch.epochTypeString()} onValueChange={setEpochSelectType}>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Day" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="DATE">Day</SelectItem>
              <SelectItem value="WEEK">Week</SelectItem>
              <SelectItem value="ALL">All</SelectItem>
            </SelectContent>
          </Select>
          {!epoch.isNullEpoch() && (
            <div className="flex items-center space-x-2">
              <Button variant="outline" size="icon" onClick={() => setEpoch(epoch.prevEpoch())}>
                <ChevronLeft className="h-4 w-4" />
              </Button>
              {epochDisplayString}
              <Button variant="outline" size="icon" onClick={() => setEpoch(epoch.nextEpoch())}>
                <ChevronRight className="h-4 w-4" />
              </Button>
            </div>
          )}
        </div>
        <TaskList
          tasks={scheduledTask!.tasks.map((t) => Task.fromGQL(t))}
          listType={ListType.Scheduled}
        />
        {!epoch.isNullEpoch() && (
          <TaskList
            tasks={plannedTasks!.tasks.map((t) => Task.fromGQL(t))}
            listType={ListType.Planned}
          />
        )}
        <Separator />
        <div className="mt-4 flex justify-center">
          <CreateTaskDialog
            onCreate={onCreateTask}
            trigger={<Button variant="outline">Create Task</Button>}
            defaultEpoch={epoch}
          ></CreateTaskDialog>
        </div>
      </div>
    </div>
  );
}
