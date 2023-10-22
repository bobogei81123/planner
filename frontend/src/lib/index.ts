import * as uuid from 'uuid';

// place files you want to import through the `$lib` alias in this folder.
export function uuidToBase64(uuidStr: string): string {
  const byteStr = String.fromCharCode(...uuid.parse(uuidStr));
  return btoa(byteStr);
}

export function base64ToUuid(base64: string): string {
  const bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
  return uuid.stringify(bytes);
}
