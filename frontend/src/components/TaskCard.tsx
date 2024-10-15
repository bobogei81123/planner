import { useMutation } from '@apollo/client';
import * as dateFns from 'date-fns';
import { useState } from 'react';

import { UpdateTaskInput } from '@/graphql/generated/graphql';
import { DELETE_TASK, UPDATE_TASK } from '@/graphql/task';
import { Task } from '@/lib/task';

import { UpdateTaskDialog } from './EditTaskDialog';
import { TodoCheckbox } from './TodoCheckbox';

interface TaskCardProps {
  task: Task;
}

export default function TaskCard({ task }: TaskCardProps) {
  const { title, cost, isCompleted } = task;
  const [isEditing, setIsEditing] = useState(false);
  const [updateTaskMutation] = useMutation(UPDATE_TASK);
  const [deleteTaskMutation] = useMutation(DELETE_TASK);

  function onUpdate(patch: Omit<UpdateTaskInput, 'id'>) {
    void updateTaskMutation({
      variables: {
        input: {
          id: task.id,
          ...patch,
        },
      },
      update(cache) {
        cache.evict({ fieldName: 'tasks' });
      },
    });
  }

  function onDelete() {
    void deleteTaskMutation({
      variables: {
        id: task.id,
      },
      update(cache) {
        cache.evict({ fieldName: 'tasks' });
      },
    });
  }

  function onCompletedChange(value: boolean) {
    void updateTaskMutation({
      variables: {
        input: {
          id: task.id,
          completeDate: value ? dateFns.format(new Date(), 'yyyy-MM-dd') : null,
        },
      },
    });
  }

  return (
    <>
      <div className="w-full h-16 flex items-center">
        <TodoCheckbox className="mx-3" checked={isCompleted} onCheckedChange={onCompletedChange} />
        <div className="grow min-w-0 truncate" onClick={() => setIsEditing(true)}>
          <span className="text-lg">{title}</span>
        </div>
        {cost != undefined && <CostSquare cost={cost} />}
      </div>
      {isEditing && (
        <UpdateTaskDialog
          task={task}
          onUpdate={onUpdate}
          onDelete={onDelete}
          onClose={() => setIsEditing(false)}
        />
      )}
    </>
  );
}

function CostSquare({ cost }: { cost: number }) {
  return (
    <div className="w-10 h-10 ml-2 bg-muted rounded-sm flex items-center justify-center shrink-0">
      <span className="text-xl">{cost}</span>
    </div>
  );
}
