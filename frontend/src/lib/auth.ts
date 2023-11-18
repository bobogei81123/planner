import { get, writable, type Writable } from "svelte/store";

const token = localStorage.getItem("jwtToken");
export const tokenStore = writable(token);

export async function login(username: String) {
  const loginResponse = await fetch('/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username,
    }),
  });

  if (loginResponse.ok) {
    const token = JSON.parse(await loginResponse.text());
    if (token.token === null) {
      return;
    }
    localStorage.setItem("jwtToken", token.token);
    tokenStore.set(token);
  }
}
