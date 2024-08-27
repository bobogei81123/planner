import { ApolloClient, ApolloProvider, InMemoryCache, createHttpLink } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { onError } from '@apollo/client/link/error';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';

import './index.css';
import { setInitialDateOptions } from './lib/date.ts';
import App from './routes/App.tsx';
import Login from './routes/Login.tsx';

setInitialDateOptions();

const httpLink = createHttpLink({
  uri: '/graphql',
});
const errorLink = onError(({ graphQLErrors, networkError }) => {
  if (graphQLErrors != null) {
    for (const error of graphQLErrors) {
      const code = error.extensions?.code;
      console.log('code =', code);
      if (code == 'UNAUTHORIZED') {
        if (router != null) {
          void router.navigate('/login');
        }
      }
    }
  }
  console.error(
    'GraphQL query failed, graphQLErrors =',
    graphQLErrors,
    ', networkError =',
    networkError,
  );
});
const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('bearerToken');
  return {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  };
});

const client = new ApolloClient({
  uri: '/graphql',
  link: authLink.concat(errorLink.concat(httpLink)),
  cache: new InMemoryCache(),
});

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
  },
  {
    path: '/login',
    element: <Login />,
  },
]);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ApolloProvider client={client}>
      <RouterProvider router={router} />
    </ApolloProvider>
  </StrictMode>,
);
