import { ChevronLeft, ChevronRight } from 'lucide-react';

import { Epoch, EpochType } from '@/lib/task';

import { Button } from './ui/button';
import { Calendar } from './ui/calendar';
import { Popover, PopoverContent, PopoverTrigger } from './ui/popover';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';

interface EpochSelectorProps {
  epoch: Epoch;
  setEpoch: (epoch: Epoch) => void;
  canSelectType?: boolean;
}

export default function EpochSelector({
  epoch,
  setEpoch,
  canSelectType = true,
}: EpochSelectorProps) {
  function setEpochSelectType(type: EpochType) {
    const currentDate = epoch.startDate() ?? new Date();
    switch (type) {
      case EpochType.Date:
        setEpoch(Epoch.ofDate(new Date()));
        break;
      case EpochType.Week:
        setEpoch(Epoch.ofWeek(currentDate));
        break;
      case EpochType.All:
        setEpoch(Epoch.nullEpoch());
        break;
    }
  }

  function setEpochDate(date?: Date) {
    if (date == undefined) return;
    switch (epoch.epochType()) {
      case EpochType.Date:
        setEpoch(Epoch.ofDate(date));
        break;
      case EpochType.Week:
        setEpoch(Epoch.ofWeek(date));
        break;
      case EpochType.All:
        console.warn('bug: `setEpochDate` called when epoch type is `ALL`');
        return;
    }
  }

  let epochDateNode;
  if (epoch.isNullEpoch()) {
    epochDateNode = <span>{epoch.toDisplayString()}</span>;
  } else {
    epochDateNode = (
      <Popover>
        <PopoverTrigger asChild>
          <span>{epoch.toDisplayString()}</span>
        </PopoverTrigger>
        <PopoverContent className="w-auto p-0">
          <Calendar
            mode="single"
            selected={epoch.startDate()}
            onSelect={setEpochDate}
            initialFocus
          />
        </PopoverContent>
      </Popover>
    );
  }

  return (
    <div className="w-full mt-2 mb-4 flex flex-col items-center space-y-2">
      {canSelectType && (
        <Select value={epoch.epochType()} onValueChange={setEpochSelectType}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Day" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="DATE">Day</SelectItem>
            <SelectItem value="WEEK">Week</SelectItem>
            <SelectItem value="ALL">All</SelectItem>
          </SelectContent>
        </Select>
      )}
      {!epoch.isNullEpoch() && (
        <div className="flex items-center space-x-2">
          <Button variant="outline" size="icon" onClick={() => setEpoch(epoch.prevEpoch())}>
            <ChevronLeft className="h-4 w-4" />
          </Button>
          {epochDateNode}
          <Button variant="outline" size="icon" onClick={() => setEpoch(epoch.nextEpoch())}>
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
      )}
    </div>
  );
}
