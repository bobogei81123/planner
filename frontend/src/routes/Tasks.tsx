import { useQuery } from '@apollo/client';
import { useContext } from 'react';

import { EditTaskDialogsContext, EditTaskDialogsProvider } from '@/components/EditTaskDialog';
import EpochSelector from '@/components/EpochSelector';
import { Button } from '@/components/ui/button.tsx';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator.tsx';
import { ViewType } from '@/graphql/generated/graphql';
import { LIST_TASKS } from '@/graphql/task';
import { EpochType, Task, useUrlParamEpoch } from '@/lib/task';
import { stringCompare } from '@/lib/utils';
import TaskListItem from '@/components/TaskListItem';


function TasksInner() {
  const editTaskDialogs = useContext(EditTaskDialogsContext);
  const [epoch, setEpoch] = useUrlParamEpoch();
  const {
    loading,
    error,
    data: gqlTasks,
  } = useQuery(LIST_TASKS, {
    variables: {
      viewType: ViewType.Scheduled,
      epoch: epoch.toGQL(),
    },
  });

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error : {error.message}</p>;

  const tasks = gqlTasks!.tasks.map((t) => Task.fromGQL(t));
  const exactlyScheduledTasks = tasks.filter(
    (t) => t.scheduledOn.epochType() === epoch.epochType(),
  );
  const otherTasks = tasks.filter((t) => t.scheduledOn.epochType() !== epoch.epochType());

  const exactlyScheduledTitle = (() => {
    switch (epoch.epochType()) {
      case EpochType.Date:
        return 'Scheduled for This Day';
      case EpochType.Week:
        return 'Scheduled for This Week';
      case EpochType.All:
        return 'Not Scheduled';
    }
  })();

  return (
    <div className="w-full h-full flex justify-center py-2">
      <div className="w-[95vw] max-w-xl h-full flex flex-col">
        <EpochSelector epoch={epoch} setEpoch={setEpoch} />
        <ScrollArea className="w-full">
          <div className="w-[95vw] max-w-xl flex flex-col items-center">
            <div className="w-full">
              <DoubleHr text={exactlyScheduledTitle} />
              <TaskList tasks={exactlyScheduledTasks} />
              <DoubleHr text="Other Tasks in Sub-iterations" />
              <TaskList tasks={otherTasks} />
            </div>
          </div>
        </ScrollArea>
        <Separator />
        <div className="mt-4 flex justify-center">
          <Button variant="outline" onClick={() => editTaskDialogs.openCreateTaskDialog(epoch)}>
            Create Task
          </Button>
        </div>
      </div>
    </div>
  );
}

interface TaskListProps {
  tasks: Task[];
}

function TaskList({ tasks }: TaskListProps) {
  const mainSection = (() => {
    if (tasks.length === 0) {
      return (
        <div className="flex justify-center my-2">
          <span className="text-lg">Nothing...</span>
        </div>
      );
    } else {
      const taskCards = tasks
        .sort((a, b) => stringCompare(a.id, b.id))
        .map((task) => <TaskListItem key={task.id} task={task}></TaskListItem>);
      return <div className="flex flex-col px-3 py-2">{taskCards}</div>;
    }
  })();

  return <>{mainSection}</>;
}

function DoubleHr({ text }: { text?: string }) {
  return (
    <div className="w-full flex justify-center items-center">
      <div className="h-[4px] grow border-t border-b border-muted-strong"></div>
      {text && (
        <>
          <span className="px-2">{text}</span>
          <div className="h-[4px] grow border-t border-b border-muted-strong"></div>
        </>
      )}
    </div>
  );
}

export default function Tasks() {
  return (
    <EditTaskDialogsProvider>
      <TasksInner />
    </EditTaskDialogsProvider>
  );
}
