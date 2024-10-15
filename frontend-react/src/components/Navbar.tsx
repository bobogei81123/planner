import { NavLink, NavLinkRenderProps } from 'react-router-dom';

import { cn } from '@/lib/utils';

import { ModeToggle } from './modeToggle';

export default function Navbar() {
  function navLinkStyle({ isActive }: NavLinkRenderProps) {
    let result;
    if (isActive) {
      result = 'bg-muted';
    } else {
      result = 'text-muted-foreground';
    }
    return cn(result, 'px-2 rounded-full transition-colors hover:text-primary');
  }

  return (
    <div className="h-12 px-2 border-b flex items-center">
      <nav className="px-6 flex items-center space-x-6">
        <NavLink to="/tasks" className={navLinkStyle}>
          Tasks
        </NavLink>
      </nav>
      <div className="ml-auto">
        <ModeToggle />
      </div>
    </div>
  );
}
