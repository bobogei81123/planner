import * as dateFns from 'date-fns';

export function setInitialDateOptions() {
  dateFns.setDefaultOptions({
    weekStartsOn: 1,
  });
}

export function formatISODate(date: Date): string {
  return dateFns.formatISO(date, { representation: 'date' });
}
