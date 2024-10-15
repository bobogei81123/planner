import { Outlet } from 'react-router-dom';

import Navbar from '@/components/Navbar';

export default function App() {
  return (
    <>
      <div className="w-full flex flex-col justify-center">
        <Navbar />
        <Outlet />
      </div>
    </>
  );
}
