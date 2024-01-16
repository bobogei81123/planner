import { addWeeks, formatISO, startOfWeek } from 'date-fns';

export class Week {
  #firstDateOfWeek: Date;

  constructor(firstDateOfWeek: Date) {
    this.#firstDateOfWeek = firstDateOfWeek;
  }

  static ofDate(date: Date): Week {
    return new Week(startOfWeek(date));
  }

  startDate(): Date {
    return this.#firstDateOfWeek;
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
