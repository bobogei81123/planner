import * as uuid from 'uuid';
import type { PageLoadEvent } from './$types';

export function load(event: PageLoadEvent) {
  const bytes = Uint8Array.from(atob(event.params.id), c => c.charCodeAt(0));
  return { id: uuid.stringify(bytes) };
}
