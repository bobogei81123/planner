import { useMutation } from '@apollo/client';
import * as dateFns from 'date-fns';
import { useState } from 'react';

import { UpdateTaskInput } from '@/graphql/generated/graphql';
import { DELETE_TASK, UPDATE_TASK } from '@/graphql/task';
import { Task } from '@/lib/task';

import { UpdateTaskDialog } from './EditTaskDialog';
import { Checkbox } from './ui/checkbox';

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
      <div className="flex items-center space-x-4">
        <Checkbox checked={isCompleted} onCheckedChange={onCompletedChange} />
        <h2 onClick={() => setIsEditing(true)}>{title}</h2>
        {cost != null && <p>[{cost}]</p>}
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
