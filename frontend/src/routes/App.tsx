import { Outlet } from 'react-router-dom';

import Navbar from '@/components/Navbar';

export default function App() {
  return (
    <div>
      <Navbar />
      <main className="w-full h-dvh pt-12">
        <Outlet />
      </main>
    </div>
  );
}
