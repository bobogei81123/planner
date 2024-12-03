import { useMutation } from '@apollo/client';
import * as dateFns from 'date-fns';
import { AlertCircle, CalendarIcon, Trash2 } from 'lucide-react';
import React, { Dispatch, SetStateAction, createContext, useState } from 'react';

import { toGQLDate } from '@/graphql/date';
import { CreateTaskInput, InputRecurringSpec, UpdateTaskInput } from '@/graphql/generated/graphql';
import { CREATE_TASK, DELETE_TASK, UPDATE_TASK } from '@/graphql/task';
import { Epoch, EpochType, Task } from '@/lib/task';
import { cn } from '@/lib/utils';

import { Alert, AlertDescription, AlertTitle } from './ui/alert';
import { Button } from './ui/button';
import { Calendar } from './ui/calendar';
import { Checkbox } from './ui/checkbox';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Popover, PopoverContent, PopoverTrigger } from './ui/popover';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';
import { Sheet, SheetContent, SheetFooter, SheetHeader, SheetTitle } from './ui/sheet';

interface EditTaskData {
  title: string;
  cost: string;
  epochType: EpochType;
  epochDate?: Date;
  recurring?: RecurringSpec;
}

type RecurringSpec = { everyNWeek: number };

interface EditTaskFormProps {
  value: EditTaskData;
  setValue: (fn: (prev: EditTaskData) => EditTaskData) => void;
  errorMessage?: string;
}

function EditTaskForm({
  value: { title, cost, epochType, epochDate, recurring },
  setValue,
  errorMessage,
}: EditTaskFormProps) {
  function wrapFn<T>(fn: (value: T) => Partial<EditTaskData>): (value: T) => void {
    return (value: T) => {
      const update = fn(value);
      setValue((prev) => ({ ...prev, ...update }));
    };
  }
  const setTitle = wrapFn((title?: string) => ({ title }));
  const setCost = wrapFn((cost?: string) => ({ cost }));
  const setEpochType = wrapFn((epochType: EpochType) => ({
    epochType,
  }));
  const setEpochDate = wrapFn((epochDate?: Date) => ({
    epochDate,
  }));
  const setRecurring = wrapFn((recurring?: RecurringSpec) => ({
    recurring,
  }));
  function onRecurringCheckChange(checked: boolean) {
    if (checked) {
      setRecurring({ everyNWeek: 1 });
    } else {
      setRecurring(undefined);
    }
  }

  return (
    <div className="grid gap-4 py-4">
      <div className="grid grid-cols-4 items-center gap-4">
        <Label htmlFor="title-input" className="text-right">
          Title
        </Label>
        <Input
          id="title-input"
          className="col-span-3"
          value={title ?? ''}
          onChange={(e) => setTitle(e.target.value)}
        />
      </div>
      <div className="grid grid-cols-4 items-center gap-4">
        <Label htmlFor="cost-input" className="text-right">
          Cost
        </Label>
        <Input
          id="cost-input"
          type="number"
          className="col-span-3"
          value={cost}
          onChange={(e) => setCost(e.target.value)}
        />
      </div>
      <div className="grid grid-cols-4 items-center gap-4">
        <Label className="text-right">Scheduled on</Label>
        <Select value={epochType} onValueChange={(ep) => setEpochType(ep as EpochType)}>
          <SelectTrigger className="col-span-3">
            <SelectValue placeholder="Day" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="DATE">Day</SelectItem>
            <SelectItem value="WEEK">Week</SelectItem>
            <SelectItem value="ALL">All</SelectItem>
          </SelectContent>
        </Select>
      </div>
      {epochType !== EpochType.All && (
        <div className="grid grid-cols-4 items-center gap-4">
          <Label className="text-right">Scheduled Date</Label>
          <Popover>
            <PopoverTrigger asChild>
              <Button
                variant={'outline'}
                className={cn(
                  'w-[280px] justify-start text-left font-normal',
                  epochDate == null && 'text-muted-foreground',
                )}
              >
                <CalendarIcon className="mr-2 h-4 w-4" />
                {epochDate != null ? dateFns.format(epochDate, 'PPP') : <span>Pick a date</span>}
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-auto p-0">
              <Calendar
                mode="single"
                selected={epochDate}
                onSelect={(date) => setEpochDate(date)}
              />
            </PopoverContent>
          </Popover>
        </div>
      )}
      <div className="grid grid-cols-4 items-center gap-4">
        <Label className="text-right">Recurring</Label>
        <Checkbox checked={recurring !== undefined} onCheckedChange={onRecurringCheckChange}>
          <SelectTrigger className="col-span-3">
            <SelectValue placeholder="Day" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="DATE">Day</SelectItem>
            <SelectItem value="WEEK">Week</SelectItem>
            <SelectItem value="ALL">All</SelectItem>
          </SelectContent>
        </Checkbox>
      </div>
      {recurring !== undefined && (
        <div className="grid grid-cols-4 items-center gap-4">
          <Label htmlFor="cost-input" className="text-right">
            Recur every N week
          </Label>
          <Input
            id="recurring-week-input"
            type="number"
            className="col-span-3"
            value={recurring.everyNWeek}
            onChange={(e) =>
              setRecurring({
                everyNWeek: e.target.value != undefined ? Number(e.target.value) : 1,
              })
            }
          />
        </div>
      )}
      {errorMessage && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertTitle>Error</AlertTitle>
          <AlertDescription>{errorMessage}</AlertDescription>
        </Alert>
      )}
    </div>
  );
}

type CreateTaskData = Omit<CreateTaskInput, 'id'>;
interface CreateTaskDialogProps {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  onCreate: (data: CreateTaskData) => void;
  defaultEpoch: Epoch;
}

export function CreateTaskDialog({ open, setOpen, onCreate, defaultEpoch }: CreateTaskDialogProps) {
  const [data, setData] = useState<EditTaskData>({
    title: '',
    cost: '',
    epochType: defaultEpoch.epochType(),
    epochDate: defaultEpoch.startDate(),
  });
  const [errorMessage, setErrorMessage] = useState<string | undefined>(undefined);

  function getCreateTaskData({
    title,
    cost,
    epochType,
    epochDate,
    recurring,
  }: EditTaskData): CreateTaskData | Error {
    if (title === '') {
      return new Error('Title is required');
    }
    let epoch;
    switch (epochType) {
      case EpochType.All:
        epoch = Epoch.nullEpoch();
        break;
      case EpochType.Date: {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofDate(epochDate);
        break;
      }
      case EpochType.Week: {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofWeek(epochDate);
        break;
      }
    }
    let recurringSpec: InputRecurringSpec | undefined = undefined;
    if (recurring !== undefined) {
      recurringSpec = {
        pattern: {
          every: recurring.everyNWeek,
        },
        startDate: toGQLDate(dateFns.startOfWeek(new Date())),
      };
    }
    const parsedCost = parseCost(cost);
    if (parsedCost instanceof Error) {
      return parsedCost;
    }

    return {
      title,
      cost: parsedCost,
      scheduledOn: epoch.toGQL(),
      recurringSpec,
    };
  }

  function onSubmitHandler() {
    const finalData = getCreateTaskData(data);
    if (finalData instanceof Error) {
      setErrorMessage(finalData.message);
      return false;
    }
    setOpen(false);
    onCreate(finalData);
  }

  function onOpenChange(value: boolean) {
    if (value) {
      clearState();
    }
    setOpen(value);
  }

  function clearState() {
    setData({
      title: '',
      cost: '',
      epochType: defaultEpoch.epochType(),
      epochDate: defaultEpoch.startDate(),
    });
    setErrorMessage(undefined);
  }

  return (
    <Sheet modal={false} open={open} onOpenChange={onOpenChange}>
      <SheetContent className="sm:max-w-[425px]">
        <SheetHeader>
          <SheetTitle>Create task</SheetTitle>
        </SheetHeader>
        <EditTaskForm value={data} setValue={setData} errorMessage={errorMessage} />
        <SheetFooter>
          <Button type="submit" onClick={onSubmitHandler}>
            Create
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  );
}

interface UpdateTaskDialogProps {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  task: Task | undefined;
  onUpdate: (data: UpdateTaskInput) => void;
  onDelete: (id: string) => void;
}

export function UpdateTaskDialog({
  open,
  setOpen,
  task,
  onUpdate,
  onDelete,
}: UpdateTaskDialogProps) {
  const taskId = task?.id ?? '';
  const [data, setData] = useState<EditTaskData>({
    title: task?.title ?? '',
    cost: task?.cost?.toString() ?? '',
    epochType: task?.scheduledOn.epochType() ?? EpochType.All,
    epochDate: task?.scheduledOn.startDate(),
  });
  const [errorMessage, setErrorMessage] = useState<string | undefined>(undefined);

  function getUpdateTaskData({
    title,
    cost,
    epochType,
    epochDate,
  }: EditTaskData): UpdateTaskInput | Error {
    if (title == undefined || title === '') {
      return new Error('Title is required');
    }
    let epoch;
    switch (epochType) {
      case EpochType.All:
        epoch = Epoch.nullEpoch();
        break;
      case EpochType.Date: {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofDate(epochDate);
        break;
      }
      case EpochType.Week: {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofWeek(epochDate);
        break;
      }
    }
    const parsedCost = parseCost(cost);
    if (parsedCost instanceof Error) {
      return parsedCost;
    }

    return {
      id: taskId,
      title,
      cost: parsedCost,
      scheduledOn: epoch.toGQL(),
    };
  }

  function onSubmitHandler() {
    const finalData = getUpdateTaskData(data);
    if (finalData instanceof Error) {
      setErrorMessage(finalData.message);
      return false;
    }
    onUpdate(finalData);
    setOpen(false);
  }

  function onDeleteHandler() {
    onDelete(taskId);
    setOpen(false);
  }

  return (
    <Sheet modal={false} open={open} onOpenChange={setOpen}>
      <SheetContent className="sm:max-w-[425px]">
        <SheetHeader>
          <SheetTitle>Update task</SheetTitle>
        </SheetHeader>
        <EditTaskForm value={data} setValue={setData} errorMessage={errorMessage} />
        <SheetFooter>
          <Button variant="destructive" onClick={onDeleteHandler}>
            <Trash2 className="mr-2 h-4 w-4" />
            Delete
          </Button>
          <Button onClick={onSubmitHandler}>Update</Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  );
}

interface EditTaskDialogsState {
  openCreateTaskDialog: (defaultEpoch: Epoch) => void;
  openUpdateTaskDialog: (task: Task) => void;
}

export const EditTaskDialogsContext = createContext<EditTaskDialogsState>({
  openCreateTaskDialog: () => undefined,
  openUpdateTaskDialog: () => undefined,
});

export function EditTaskDialogsProvider({ children }: { children: React.ReactNode }) {
  const [renderKey, setRenderKey] = useState(0);
  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);
  const [createDialogState, setCreateDialogState] = useState({
    defaultEpoch: Epoch.nullEpoch(),
  });
  const [updateDialogOpen, setUpdateDialogOpen] = useState<boolean>(false);
  const [updateDialogState, setUpdateDialogState] = useState({
    task: undefined as Task | undefined,
  });
  const [createTaskMutation] = useMutation(CREATE_TASK);
  const [updateTaskMutation] = useMutation(UPDATE_TASK);
  const [deleteTaskMutation] = useMutation(DELETE_TASK);

  function onCreate(data: CreateTaskInput) {
    void createTaskMutation({
      variables: data,
      update(cache) {
        cache.evict({ fieldName: 'tasks' });
      },
    });
  }

  function onUpdate(data: UpdateTaskInput) {
    void updateTaskMutation({
      variables: {
        input: data,
      },
      update(cache) {
        cache.evict({ fieldName: 'tasks' });
      },
    });
  }

  function onDelete(id: string) {
    void deleteTaskMutation({
      variables: {
        id,
      },
      update(cache) {
        cache.evict({ fieldName: 'tasks' });
      },
    });
  }

  const state: EditTaskDialogsState = {
    openCreateTaskDialog: (defaultEpoch: Epoch) => {
      setRenderKey(renderKey + 1);
      setCreateDialogState({ defaultEpoch });
      setCreateDialogOpen(true);
      setUpdateDialogOpen(false);
    },
    openUpdateTaskDialog: (task: Task) => {
      setRenderKey(renderKey + 1);
      setUpdateDialogState({ task });
      setUpdateDialogOpen(true);
      setCreateDialogOpen(false);
    },
  };

  return (
    <EditTaskDialogsContext.Provider value={state}>
      <CreateTaskDialog
        key={renderKey * 2}
        open={createDialogOpen}
        setOpen={setCreateDialogOpen}
        onCreate={onCreate}
        {...createDialogState}
      />
      <UpdateTaskDialog
        key={renderKey * 2 + 1}
        open={updateDialogOpen}
        setOpen={setUpdateDialogOpen}
        onUpdate={onUpdate}
        onDelete={onDelete}
        {...updateDialogState}
      />
      {children}
    </EditTaskDialogsContext.Provider>
  );
}

function parseCost(cost: string): number | null | Error {
  if (cost === '') {
    return null;
  }
  const parsedCost = Number(cost);
  if (Number.isNaN(parsedCost)) {
    return new Error('Cost must be a number');
  }
  return parsedCost;
}
