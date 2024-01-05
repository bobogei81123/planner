import { get, writable } from 'svelte/store';
import type { AuthConfig, AuthUtilities } from '@urql/exchange-auth';
import { goto } from '$app/navigation';

const token = localStorage.getItem('jwtToken');
export const tokenStore = writable(token);

export async function login(username: string) {
  const loginResponse = await fetch('/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      username
    })
  });

  if (loginResponse.ok) {
    const token = JSON.parse(await loginResponse.text());
    if (token.token === null) {
      return;
    }
    localStorage.setItem('jwtToken', token.token);
    tokenStore.set(token.token);
    console.info('Logged in');
  }
}

export function getAuthConfig(utils: AuthUtilities): AuthConfig {
  return {
    addAuthToOperation(operation) {
      const token = get(tokenStore);
      if (token === null) {
        return operation;
      }
      return utils.appendHeaders(operation, {
        Authorization: `Bearer ${token}`
      });
    },
    didAuthError(error) {
      return error.graphQLErrors.some((e) => e.extensions?.code === 'FORBIDDEN');
    },
    async refreshAuth() {
      await goto('/login');
    }
  };
}
