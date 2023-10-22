import * as uuid from 'uuid';
import type { PageLoadEvent } from './$types';

export function load({ params }: PageLoadEvent) {
  const bytes = Uint8Array.from(atob(params.id), c => c.charCodeAt(0));
  return { id: uuid.stringify(bytes) };
}
