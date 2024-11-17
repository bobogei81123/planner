import { useMutation } from '@apollo/client';
import * as dateFns from 'date-fns';
import { useContext } from 'react';

import { UPDATE_TASK_COMPLETE_DATE } from '@/graphql/task';
import { Task } from '@/lib/task';

import { EditTaskDialogsContext } from './EditTaskDialog';
import { CostSquare, TodoCheckbox } from './TaskUIComponents';

interface TaskListItemProps {
  task: Task;
}

export default function TaskListItem({ task }: TaskListItemProps) {
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
    <>
      <div className="w-full h-16 flex items-center">
        <div className="w-10 h-10 mx-3 shrink-0">
          <TodoCheckbox checked={isCompleted} onCheckedChange={onCompletedChange} />
        </div>
        <div
          className="grow min-w-0 truncate"
          onClick={() => editTaskDialogs.openUpdateTaskDialog(task)}
        >
          <span className="text-lg">{title}</span>
        </div>
        {cost != undefined && (
          <div className="w-10 h-10 ml-2 shrink-0">
            <CostSquare cost={cost} />
          </div>
        )}
      </div>
    </>
  );
}
