import * as dateFns from 'date-fns';

import { fromGQLDate, toGQLDate } from '@/graphql/date';
import {
  EpochType,
  Epoch as GQLEpoch,
  Task as GQLTask,
  InputEpoch,
} from '@/graphql/generated/graphql';

interface DatedEpoch {
  type: EpochType;
  date: Date;
}

type EpochInner = DatedEpoch | null;
export type EpochTypeString = 'DATE' | 'WEEK' | 'ALL';

export class Epoch {
  readonly inner: EpochInner;

  constructor(inner: EpochInner) {
    this.inner = inner;
  }

  get date(): Date | null {
    return this.inner?.date ?? null;
  }

  get type(): EpochType | null {
    return this.inner?.type ?? null;
  }

  static nullEpoch(): Epoch {
    return new Epoch(null);
  }

  static ofDate(date: Date): Epoch {
    return new Epoch({
      type: EpochType.Date,
      date,
    });
  }

  static ofWeek(date: Date): Epoch {
    return new Epoch({
      type: EpochType.Week,
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
      case EpochType.Date:
        return this.inner.date;
      case EpochType.Week:
        return dateFns.startOfWeek(this.inner.date);
    }
  }

  nextEpoch(): Epoch {
    switch (this.inner?.type) {
      case undefined:
        return Epoch.nullEpoch();
      case EpochType.Date:
        return Epoch.ofDate(dateFns.addDays(this.inner.date, 1));
      case EpochType.Week:
        return Epoch.ofWeek(dateFns.addWeeks(this.inner.date, 1));
    }
  }

  prevEpoch(): Epoch {
    switch (this.inner?.type) {
      case undefined:
        return Epoch.nullEpoch();
      case EpochType.Date:
        return Epoch.ofDate(dateFns.subDays(this.inner.date, 1));
      case EpochType.Week:
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

  epochTypeString(): EpochTypeString {
    switch (this.inner?.type) {
      case undefined:
        return 'ALL';
      case EpochType.Date:
        return 'DATE';
      case EpochType.Week:
        return 'WEEK';
    }
  }

  isNullEpoch(): boolean {
    return this.inner === null;
  }

  toDisplayString(): string {
    switch (this.inner?.type) {
      case undefined:
        return 'All Time';
      case EpochType.Date:
        return dateFns.format(this.inner.date, 'PPP');
      case EpochType.Week: {
        const EM_DASH = '\u2013';
        const start = dateFns.startOfWeek(this.inner.date);
        const end = dateFns.endOfWeek(this.inner.date);
        return `${dateFns.format(start, 'PP')} ${EM_DASH} ${dateFns.format(end, 'PP')}`;
      }
    }
  }
}

export class Task {
  constructor(
    readonly id: string,
    readonly title: string,
    readonly isCompleted: boolean,
    readonly cost?: number,
    readonly scheduledOn: Epoch = Epoch.nullEpoch(),
  ) {}

  static fromGQL(task: GQLTask): Task {
    return new Task(
      task.id,
      task.title,
      task.isCompleted,
      task.cost ?? undefined,
      Epoch.fromGQL(task.scheduledOn ?? null),
    );
  }

  // static fromObj({
  //   title,
  //   isCompleted,
  //   cost,
  //   scheduleOn,
  // }: {
  //   title: string;
  //   isCompleted: boolean;
  //   cost?: number | null;
  //   scheduleOn?: Epoch;
  // }) {
  //   return new Task(title, isCompleted, cost, scheduleOn);
  // }
}
