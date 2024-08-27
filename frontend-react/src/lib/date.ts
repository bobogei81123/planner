import * as dateFns from 'date-fns';

export function setInitialDateOptions() {
  dateFns.setDefaultOptions({
    weekStartsOn: 1,
  })
}
