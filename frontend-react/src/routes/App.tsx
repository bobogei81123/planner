import { useMutation, useQuery } from '@apollo/client';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useState } from 'react';

import { CreateTaskDialog } from '@/components/EditTaskDialog';
import TaskCard from '@/components/TaskCard.tsx';
import { Button } from '@/components/ui/button.tsx';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Separator } from '@/components/ui/separator.tsx';
import {
  CreateTaskInput,
  Task as GQLTask,
  ViewType,
} from '@/graphql/generated/graphql';
import { CREATE_TASK, LIST_TASKS } from '@/graphql/task.ts';
import { Epoch, EpochTypeString, Task } from '@/lib/task';

export default function App() {
  const [epoch, setEpoch] = useState<Epoch>(Epoch.ofDate(new Date()));
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

  if (scheduledTasksLoading || plannedTasksLoading) return <p>Loading...</p>;
  if (scheduledTaskError || plannedTasksError) return <p>Error : {scheduledTaskError?.message}</p>;

  function createTaskCard(gqlTask: GQLTask) {
    const task = Task.fromGQL(gqlTask);
    return (
      <TaskCard
        key={gqlTask.id}
        task={task}
      ></TaskCard>
    );
  }

  const tasks = scheduledTask!.tasks.map(createTaskCard);
  const plannedTasksView = plannedTasks!.tasks.map(createTaskCard);

  function onCreateTask(data: CreateTaskInput) {
    void createTaskMutation({
      variables: data,
    });
  }

  return (
    <>
      <div className="w-full flex justify-center">
        <div className="w-1/2 bg-slate-200 h-screen flex-col">
          <div className="flex flex-col w-full bg-slate-400 h-32 items-center py-2 space-y-2">
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
                <span>{epoch.toDisplayString()}</span>
                <Button variant="outline" size="icon" onClick={() => setEpoch(epoch.nextEpoch())}>
                  <ChevronRight className="h-4 w-4" />
                </Button>
              </div>
            )}
          </div>
          <div className="flex flex-col bg-slate-300 px-3 py-2">{tasks}</div>
          <Separator />
          <div className="flex flex-col bg-slate-300 px-3 py-2">{plannedTasksView}</div>
          <Separator />
          <div className="flex justify-center">
            <CreateTaskDialog
              onCreate={onCreateTask}
              trigger={<Button variant="outline">Create Task</Button>}
              defaultEpoch={epoch}
            ></CreateTaskDialog>
          </div>
        </div>
      </div>
    </>
  );
}

// function createTaskCard(task: Task, onTaskUpdate) {
//   return (
//     <TaskCard
//       key={task.id}
//       title={task.title}
//       cost={task.cost}
//       isCompleted={task.isCompleted}
//       scheduledOn={Epoch.fromGQL(task.scheduledOn ?? null)}
//       onTaskUpdate={(data) => updateTaskCallback(task.id, data)}
//       onTaskDelete={() => deleteTaskMutation({ variables: { id: task.id } })}
//     ></TaskCard>
//   );
// }
//
// function TaskList(epoch: Epoch, viewType: ViewType.Planned | ViewType.Scheduled) {}
//
