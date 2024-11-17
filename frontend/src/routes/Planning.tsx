import { useMutation, useQuery } from '@apollo/client';
import { DragDropContext, DropResult, Droppable } from '@hello-pangea/dnd';
import { Plus } from 'lucide-react';
import React, { forwardRef, useContext } from 'react';

import { EditTaskDialogsContext, EditTaskDialogsProvider } from '@/components/EditTaskDialog';
import EpochSelector from '@/components/EpochSelector';
import { TaskCard } from '@/components/TaskCard';
import { ScrollArea } from '@/components/ui/scroll-area';
import { gql } from '@/graphql/generated';
import { ViewType } from '@/graphql/generated/graphql';
import { LIST_TASKS } from '@/graphql/task';
import { Epoch, EpochType, Task, useUrlParamEpoch } from '@/lib/task';
import { stringCompare } from '@/lib/utils';

const UPDATE_TASK_EPOCH = gql(`
  mutation UpdateTaskEpoch($id: UUID!, $scheduledOn: InputEpoch) {
    updateTask(input: {id: $id, scheduledOn: $scheduledOn}) {
      id
      scheduledOn {
        type
        date
      }
    }
  }
`);

function PlanningInner() {
  const editTaskDialogs = useContext(EditTaskDialogsContext);
  const [epoch, setEpoch] = useUrlParamEpoch();
  const {
    loading: listTasksLoading,
    error: listTaskError,
    data: gqlTasks,
  } = useQuery(LIST_TASKS, {
    variables: {
      viewType: ViewType.Planned,
      epoch: epoch.toGQL(),
    },
    notifyOnNetworkStatusChange: true,
  });
  const [updateTaskEpochMutation] = useMutation(UPDATE_TASK_EPOCH);

  if (listTaskError) {
    return 'error';
  }
  if (listTasksLoading) {
    return 'loading';
  }

  const allTasks = gqlTasks!.tasks
    .map((t) => Task.fromGQL(t))
    .sort((t1, t2) => stringCompare(t1.id, t2.id));

  function getEpochOfType(epochType: EpochType): Epoch {
    const epochDate = epoch.startDate()!;
    switch (epochType) {
      case EpochType.Date:
        return Epoch.ofDate(epochDate);
      case EpochType.Week:
        return Epoch.ofWeek(epochDate);
      case EpochType.All:
        return Epoch.nullEpoch();
    }
  }

  function taskListView(epochType: EpochType) {
    const epoch = getEpochOfType(epochType);
    const tasks = allTasks.filter((t) => t.scheduledOn.epochType() === epochType);

    let name;
    switch (epochType) {
      case EpochType.Date:
        name = 'This Day';
        break;
      case EpochType.Week:
        name = 'This Week';
        break;
      case EpochType.All:
        name = 'Not Scheduled';
        break;
    }

    return (
      <div className="w-[40vw] md:w-64 h-full flex flex-col bg-primary-foreground border border-secondary rounded-md">
        <div className="w-full h-12 border-b-[3px] border-secondary flex items-center justify-center">
          <span>{name}</span>
        </div>
        <ScrollArea className="w-full">
          <Droppable droppableId={epochType}>
            {(provided) => (
              <div
                className="w-[40vw] md:w-64 flex flex-col items-center px-2 py-2 space-y-2"
                ref={provided.innerRef}
                {...provided.droppableProps}
              >
                {tasks.map((task, index) => (
                  <TaskCard task={task} index={index} key={task.id} />
                ))}
                {provided.placeholder}
                <AddTaskButton onClick={() => editTaskDialogs.openCreateTaskDialog(epoch)} />
              </div>
            )}
          </Droppable>
        </ScrollArea>
      </div>
    );
  }

  function updateTaskEpoch(taskID: string, newEpoch: Epoch) {
    const scheduledOn = newEpoch.toGQL();
    void updateTaskEpochMutation({
      variables: {
        id: taskID,
        scheduledOn,
      },
      optimisticResponse: {
        updateTask: {
          __typename: 'Task',
          id: taskID,
          scheduledOn: newEpoch.toGQL(),
        },
      },
    });
  }

  function onDragEnd({ draggableId: taskId, destination }: DropResult) {
    if (destination == null) return;
    const newEpoch = getEpochOfType(destination.droppableId as EpochType);
    updateTaskEpoch(taskId, newEpoch);
  }

  const AddTaskButton = forwardRef<HTMLButtonElement, React.HTMLAttributes<HTMLButtonElement>>(
    function (props, ref) {
      return (
        <button
          className="w-full mx-2 h-10 border border-secondary rounded-md flex justify-center items-center hover:bg-black/20"
          type="button"
          ref={ref}
          {...props}
        >
          <Plus />
        </button>
      );
    },
  );
  AddTaskButton.displayName = 'AddTaskButton';

  return (
    <div className="h-full pt-4 flex justify-center items-center flex-col">
      <EpochSelector epoch={epoch} setEpoch={setEpoch} />
      <DragDropContext onDragEnd={onDragEnd}>
        <div className="w-full overflow-x-auto flex grow shrink min-h-0">
          <div className="w-fit px-4 py-4 flex grow shrink min-h-0 justify-center flex-row space-x-8 md:space-x-10">
            {[EpochType.Date, EpochType.Week, EpochType.All].map(taskListView)}
          </div>
        </div>
      </DragDropContext>
    </div>
  );
}

export default function Planning() {
  return (
    <EditTaskDialogsProvider>
      <PlanningInner />
    </EditTaskDialogsProvider>
  );
}
