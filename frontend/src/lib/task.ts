import * as dateFns from 'date-fns';
import { useSearchParams } from 'react-router-dom';

import { fromGQLDate, toGQLDate } from '@/graphql/date';
import {
  Epoch as GQLEpoch,
  EpochType as GQLEpochType,
  Task as GQLTask,
  InputEpoch,
} from '@/graphql/generated/graphql';

import { formatISODate } from './date';

interface DatedEpoch {
  type: GQLEpochType;
  date: Date;
}

type EpochInner = DatedEpoch | null;

export enum EpochType {
  Date = 'DATE',
  Week = 'WEEK',
  All = 'ALL',
}

export class Epoch {
  readonly inner: EpochInner;

  constructor(inner: EpochInner) {
    this.inner = inner;
  }

  get date(): Date | null {
    return this.inner?.date ?? null;
  }

  get type(): GQLEpochType | null {
    return this.inner?.type ?? null;
  }

  get epoch(): GQLEpoch | undefined {
    if (this.inner === null) return undefined;
    return {
      type: this.inner.type,
      date: toGQLDate(this.inner.date),
    };
  }

  static nullEpoch(): Epoch {
    return new Epoch(null);
  }

  static ofDate(date: Date): Epoch {
    return new Epoch({
      type: GQLEpochType.Date,
      date,
    });
  }

  static ofWeek(date: Date): Epoch {
    return new Epoch({
      type: GQLEpochType.Week,
      date: dateFns.startOfWeek(date),
    });
  }

  static fromGQL(epoch: GQLEpoch | InputEpoch | null): Epoch {
    if (epoch === null) return Epoch.nullEpoch();
    return new Epoch({
      type: epoch.type,
      date: fromGQLDate(epoch.date),
    });
  }

  startDate(): Date | undefined {
    switch (this.inner?.type) {
      case undefined:
        return undefined;
      case GQLEpochType.Date:
        return this.inner.date;
      case GQLEpochType.Week:
        return dateFns.startOfWeek(this.inner.date);
    }
  }

  nextEpoch(): Epoch {
    switch (this.inner?.type) {
      case undefined:
        return Epoch.nullEpoch();
      case GQLEpochType.Date:
        return Epoch.ofDate(dateFns.addDays(this.inner.date, 1));
      case GQLEpochType.Week:
        return Epoch.ofWeek(dateFns.addWeeks(this.inner.date, 1));
    }
  }

  prevEpoch(): Epoch {
    switch (this.inner?.type) {
      case undefined:
        return Epoch.nullEpoch();
      case GQLEpochType.Date:
        return Epoch.ofDate(dateFns.subDays(this.inner.date, 1));
      case GQLEpochType.Week:
        return Epoch.ofWeek(dateFns.subWeeks(this.inner.date, 1));
    }
  }

  toGQL(): (GQLEpoch & InputEpoch) | null {
    if (this.inner === null) return null;
    return {
      type: this.inner.type,
      date: toGQLDate(this.inner.date),
    };
  }

  epochType(): EpochType {
    switch (this.inner?.type) {
      case undefined:
        return EpochType.All;
      case GQLEpochType.Date:
        return EpochType.Date;
      case GQLEpochType.Week:
        return EpochType.Week;
    }
  }

  isNullEpoch(): boolean {
    return this.inner === null;
  }

  toDisplayString(): string {
    switch (this.inner?.type) {
      case undefined:
        return 'All Time';
      case GQLEpochType.Date:
        return dateFns.format(this.inner.date, 'PPP');
      case GQLEpochType.Week: {
        const EM_DASH = '\u2013';
        const start = dateFns.startOfWeek(this.inner.date);
        const end = dateFns.endOfWeek(this.inner.date);
        return `${dateFns.format(start, 'PP')} ${EM_DASH} ${dateFns.format(end, 'PP')}`;
      }
    }
  }

  toUrlParam(): string {
    switch (this.inner?.type) {
      case undefined:
        return 'ALL';
      case GQLEpochType.Date:
        return 'DATE.' + formatISODate(this.startDate()!);
      case GQLEpochType.Week:
        return 'WEEK.' + formatISODate(this.startDate()!);
    }
  }

  static fromUrlParam(param: string): Epoch | undefined {
    if (param === 'ALL') return Epoch.nullEpoch();
    const split = param.split('.');
    if (split.length !== 2) return undefined;
    const [type, dateStr] = split;
    const date = dateFns.parseISO(dateStr);
    if (type === 'DATE') return Epoch.ofDate(date);
    if (type === 'WEEK') return Epoch.ofWeek(date);
    return undefined;
  }
}

export class Task {
  constructor(
    readonly id: string,
    readonly title: string,
    readonly isCompleted: boolean,
    readonly cost?: number,
    readonly scheduledOn: Epoch = Epoch.nullEpoch(),
    readonly recurring: unknown = undefined,
  ) {}

  static fromGQL(task: GQLTask): Task {
    return new Task(
      task.id,
      task.title,
      task.isCompleted,
      task.cost ?? undefined,
      Epoch.fromGQL(task.scheduledOn ?? null),
      task.recurring == undefined ? undefined : task.recurring,
    );
  }

  isRecurring(): boolean {
    return this.recurring != undefined;
  }
}

export function useUrlParamEpoch(): [Epoch, (epoch: Epoch) => void] {
  const [searchParams, setSearchParams] = useSearchParams();

  function tryParseEpoch(epochParam: string | null): Epoch | undefined {
    if (epochParam == null) return undefined;
    return Epoch.fromUrlParam(epochParam);
  }

  let epoch = tryParseEpoch(searchParams.get('epoch'));
  if (epoch === undefined) {
    epoch = Epoch.ofDate(new Date());
    setSearchParams({ ...searchParams, epoch: epoch.toUrlParam() });
  }
  const setEpoch = (epoch: Epoch) => {
    setSearchParams({ epoch: epoch.toUrlParam() });
  };
  return [epoch, setEpoch] as const;
}
