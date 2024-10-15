// import { useState } from 'react';
import { useSearchParams } from 'react-router-dom';

import { Epoch, Task } from '@/lib/task';

import TaskCard from './TaskCard';

interface TaskCardProps {
  tasks: Task[];
  listType: ListType;
}

export enum ListType {
  Scheduled = 'SCHEDULED',
  Planned = 'PLANNED',
}

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

export default function TaskList({ tasks, listType }: TaskCardProps) {
  const [epoch] = useEpoch();

  const title = (() => {
    if (listType === ListType.Planned) {
      return 'Can Be Scheduled';
    }
    switch (epoch.epochTypeString()) {
      case 'WEEK':
        return 'Scheduled for This Week';
      case 'DATE':
        return 'Scheduled for Today';
      default:
        return '';
    }
  })();

  let mainSection;

  if (tasks.length === 0) {
    mainSection = (
      <div className="flex justify-center my-2">
        <span className="text-lg">Nothing...</span>
      </div>
    );
  } else {
    const taskCards = tasks
      .sort((a, b) => stringCompare(a.id, b.id))
      .map((task) => <TaskCard key={task.id} task={task}></TaskCard>);
    mainSection = <div className="flex flex-col px-3 py-2">{taskCards}</div>;
  }

  return (
    <>
      {!epoch.isNullEpoch() && <DoubleHr text={title} />}
      {mainSection}
    </>
  );
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

function stringCompare(a: string, b: string) {
  if (a < b) {
    return -1;
  } else if (a > b) {
    return 1;
  } else {
    return 0;
  }
}
