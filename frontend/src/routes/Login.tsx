import { useNavigate } from 'react-router-dom';

import { Button } from '@/components/ui/button';

interface LoginResponse {
  token: string;
}

export default function Login() {
  const navigate = useNavigate();
  async function doFetch() {
    const response = await fetch('/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: 'meteor' }),
    });
    const responseJson = (await response.json()) as LoginResponse;
    if (responseJson.token != null) {
      localStorage.setItem('bearerToken', responseJson.token);
      navigate('/');
    }
  }
  return (
    <Button
      onClick={() => {
        void doFetch();
      }}
    >
      Login
    </Button>
  );
}
