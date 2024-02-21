import { addWeeks, endOfWeek, formatISO, startOfWeek } from 'date-fns';

export class Week {
  #firstDateOfWeek: Date;

  constructor(firstDateOfWeek: Date) {
    this.#firstDateOfWeek = firstDateOfWeek;
  }

  static ofDate(date: Date): Week {
    return new Week(startOfWeek(date, { weekStartsOn: 1 }));
  }

  startDate(): Date {
    return this.#firstDateOfWeek;
  }

  lastDate(): Date {
    return endOfWeek(this.#firstDateOfWeek, { weekStartsOn: 1 });
  }

  endDate(): Date {
    return addWeeks(this.#firstDateOfWeek, 1);
  }

  nextWeek(): Week {
    return Week.ofDate(addWeeks(this.#firstDateOfWeek, 1));
  }

  prevWeek(): Week {
    return Week.ofDate(addWeeks(this.#firstDateOfWeek, -1));
  }
}

export function formatISODate(date: Date): string {
  return formatISO(date, { representation: 'date' });
}

export const ALL_WEEK_DAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
