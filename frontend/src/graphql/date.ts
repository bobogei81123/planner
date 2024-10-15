import * as dateFns from 'date-fns';

export function toGQLDate(date: Date): string {
  return dateFns.format(date, 'yyyy-MM-dd');
}

export function fromGQLDate(date: string): Date {
  return dateFns.parse(date, 'yyyy-MM-dd', new Date());
}
