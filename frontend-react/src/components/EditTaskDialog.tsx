import * as dateFns from 'date-fns';
import { AlertCircle, CalendarIcon, Trash2 } from 'lucide-react';
import { useState } from 'react';

import { CreateTaskInput, UpdateTaskInput } from '@/graphql/generated/graphql';
import { Epoch, EpochTypeString, Task } from '@/lib/task';
import { cn } from '@/lib/utils';

import { Alert, AlertDescription, AlertTitle } from './ui/alert';
import { Button } from './ui/button';
import { Calendar } from './ui/calendar';
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from './ui/dialog';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Popover, PopoverContent, PopoverTrigger } from './ui/popover';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';

interface EditTaskData {
  title?: string;
  cost?: number;
  epochTypeString: EpochTypeString;
  epochDate?: Date;
}

interface EditTaskFormProps {
  defaultValue: EditTaskData;
  updateValue: (fn: (prev: EditTaskData) => EditTaskData) => void;
  errorMessage?: string;
}

function EditTaskForm({
  defaultValue: { title, cost, epochTypeString, epochDate },
  updateValue,
  errorMessage,
}: EditTaskFormProps) {
  const setTitle = (title?: string) => {
    updateValue((prev) => {
      return {
        ...prev,
        title,
      };
    });
  };
  const setCost = (cost?: number) => {
    updateValue((prev) => {
      return {
        ...prev,
        cost,
      };
    });
  };
  const setEpochTypeString = (epochTypeString: EpochTypeString) => {
    updateValue((prev) => {
      return {
        ...prev,
        epochTypeString,
      };
    });
  };
  const setEpochDate = (epochDate?: Date) => {
    updateValue((prev) => {
      return {
        ...prev,
        epochDate,
      };
    });
  };

  return (
    <div className="grid gap-4 py-4">
      <div className="grid grid-cols-4 items-center gap-4">
        <Label htmlFor="title-input" className="text-right">
          Title
        </Label>
        <Input
          id="title-input"
          className="col-span-3"
          defaultValue={title}
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
          defaultValue={cost}
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

    return {
      title,
      cost,
      scheduledOn: epoch.toGQL(),
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
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogTrigger asChild>{trigger}</DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create task</DialogTitle>
        </DialogHeader>
        <EditTaskForm defaultValue={data} updateValue={setData} errorMessage={errorMessage} />
        <DialogFooter>
          <Button type="submit" onClick={onSubmitHandler}>
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
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
    <Dialog open={true} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create task</DialogTitle>
        </DialogHeader>
        <EditTaskForm defaultValue={data} updateValue={setData} errorMessage={errorMessage} />
        <DialogFooter>
          <Button variant="destructive" onClick={onDeleteHandler}>
            <Trash2 className="mr-2 h-4 w-4" />
            Delete
          </Button>
          <Button onClick={onSubmitHandler}>Update</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
