import * as dateFns from 'date-fns';
import { AlertCircle, CalendarIcon, Trash2 } from 'lucide-react';
import { useState } from 'react';

import { toGQLDate } from '@/graphql/date';
import { CreateTaskInput, InputRecurringSpec, UpdateTaskInput } from '@/graphql/generated/graphql';
import { Epoch, EpochTypeString, Task } from '@/lib/task';
import { cn } from '@/lib/utils';

import { Alert, AlertDescription, AlertTitle } from './ui/alert';
import { Button } from './ui/button';
import { Calendar } from './ui/calendar';
import { Checkbox } from './ui/checkbox';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Popover, PopoverContent, PopoverTrigger } from './ui/popover';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';
import {
  Sheet,
  SheetContent,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from './ui/sheet';

interface EditTaskData {
  title?: string;
  cost?: number;
  epochTypeString: EpochTypeString;
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
  value: { title, cost, epochTypeString, epochDate, recurring },
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
  const setCost = wrapFn((cost?: number) => ({ cost }));
  const setEpochTypeString = wrapFn((epochTypeString: EpochTypeString) => ({
    epochTypeString,
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
          value={title}
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
          onChange={(e) =>
            setCost(e.target.value != undefined ? Number(e.target.value) : undefined)
          }
        />
      </div>
      <div className="grid grid-cols-4 items-center gap-4">
        <Label className="text-right">Scheduled on</Label>
        <Select
          value={epochTypeString}
          onValueChange={(ep) => setEpochTypeString(ep as EpochTypeString)}
        >
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
      {epochTypeString !== 'ALL' && (
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
  onCreate: (data: CreateTaskData) => void;
  trigger: React.ReactNode;
  defaultEpoch: Epoch;
}

export function CreateTaskDialog({ onCreate, trigger, defaultEpoch }: CreateTaskDialogProps) {
  const [open, setOpen] = useState(false);
  const [data, setData] = useState<EditTaskData>({
    epochTypeString: defaultEpoch.epochTypeString(),
    epochDate: defaultEpoch.startDate(),
  });
  const [errorMessage, setErrorMessage] = useState<string | undefined>(undefined);

  function getCreateTaskData({
    title,
    cost,
    epochTypeString,
    epochDate,
    recurring,
  }: EditTaskData): CreateTaskData | Error {
    if (title == undefined || title === '') {
      return new Error('Title is required');
    }
    let epoch;
    switch (epochTypeString) {
      case 'ALL':
        epoch = Epoch.nullEpoch();
        break;
      case 'DATE': {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofDate(epochDate);
        break;
      }
      case 'WEEK': {
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

    return {
      title,
      cost,
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
      epochTypeString: defaultEpoch.epochTypeString(),
      epochDate: defaultEpoch.startDate(),
    });
    setErrorMessage(undefined);
  }

  return (
    <Sheet modal={false} open={open} onOpenChange={onOpenChange}>
      <SheetTrigger asChild>{trigger}</SheetTrigger>
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
  task: Task;
  onClose: () => void;
  onUpdate: (data: Omit<UpdateTaskInput, 'id'>) => void;
  onDelete: () => void;
}

export function UpdateTaskDialog({ task, onClose, onUpdate, onDelete }: UpdateTaskDialogProps) {
  const [data, setData] = useState<EditTaskData>({
    title: task.title,
    cost: task.cost ?? undefined,
    epochTypeString: task.scheduledOn.epochTypeString(),
    epochDate: task.scheduledOn.startDate(),
  });
  const [errorMessage, setErrorMessage] = useState<string | undefined>(undefined);

  type UpdateTaskData = Omit<UpdateTaskInput, 'id'>;
  function getUpdateTaskData({
    title,
    cost,
    epochTypeString,
    epochDate,
  }: EditTaskData): UpdateTaskData | Error {
    if (title == undefined || title === '') {
      return new Error('Title is required');
    }
    let epoch;
    switch (epochTypeString) {
      case 'ALL':
        epoch = Epoch.nullEpoch();
        break;
      case 'DATE': {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofDate(epochDate);
        break;
      }
      case 'WEEK': {
        if (epochDate == null) {
          return new Error('When schedule type is not "ALL", a date is required');
        }
        epoch = Epoch.ofWeek(epochDate);
        break;
      }
    }

    return {
      title,
      cost,
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
    onClose();
  }

  function onDeleteHandler() {
    onDelete();
    onClose();
  }

  function onOpenChange(value: boolean) {
    if (value) {
      console.warn('UpdateTaskDialog should never be opened by itself');
      return;
    }
    onClose();
  }

  return (
    <Sheet modal={false} open={true} onOpenChange={onOpenChange}>
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
