import { useMutation } from '@apollo/client';
import { Draggable } from '@hello-pangea/dnd';
import * as dateFns from 'date-fns';
import { GripVertical, Repeat } from 'lucide-react';
import { useContext } from 'react';

import { UPDATE_TASK_COMPLETE_DATE } from '@/graphql/task';
import { Task } from '@/lib/task';

import { EditTaskDialogsContext } from './EditTaskDialog';
import { CostSquare, TodoCheckbox } from './TaskUIComponents';
import { Badge } from './ui/badge';
import { Button } from './ui/button';

export function TaskCard({ task, index }: { task: Task; index: number }) {
  const editTaskDialogs = useContext(EditTaskDialogsContext);
  const { title, cost, isCompleted } = task;
  const [updateTaskCompleteDate] = useMutation(UPDATE_TASK_COMPLETE_DATE);

  function onCompletedChange(value: boolean) {
    void updateTaskCompleteDate({
      variables: {
        id: task.id,
        completeDate: value ? dateFns.format(new Date(), 'yyyy-MM-dd') : null,
      },
    });
  }

  return (
    <Draggable draggableId={task.id} index={index}>
      {(provided) => (
        <div
          className="w-full mx-2 rounded-lg border bg-card text-card-foreground shadow-sm"
          onClick={() => editTaskDialogs.openUpdateTaskDialog(task)}
          ref={provided.innerRef}
          {...provided.draggableProps}
        >
          <div className="h-10 flex items-center border-b">
            <div className="flex flex-grow items-center space-x-2 pl-2">
              <div className="h-6 w-6">
                <TodoCheckbox checked={isCompleted} onCheckedChange={onCompletedChange} />
              </div>
              {cost != undefined && (
                <div className="h-6 w-6 ml-2">
                  <CostSquare cost={cost} />
                </div>
              )}
              {task.isRecurring() && (
                <Badge variant="outline">
                  <Repeat className="h-3 w-3 mr-1" />
                  Recurring
                </Badge>
              )}
            </div>
            <Button
              variant="ghost"
              className="ml-auto w-8 h-10 px-1 py-1"
              {...provided.dragHandleProps}
            >
              <GripVertical className="h-4 w-4" />
            </Button>
          </div>
          <div className="w-full h-10 flex items-center px-2">
            <span className="truncate">{title}</span>
          </div>
        </div>
      )}
    </Draggable>
  );
}
