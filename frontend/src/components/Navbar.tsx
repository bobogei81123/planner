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
    <div className="absolute h-12 w-full px-2 border-b flex items-center shrink-0">
      <nav className="px-6 flex items-center space-x-6">
        <NavLink to="/tasks" className={navLinkStyle}>
          Tasks
        </NavLink>
        <NavLink to="/planning" className={navLinkStyle}>
          Planning
        </NavLink>
      </nav>
      <div className="ml-auto">
        <ModeToggle />
      </div>
    </div>
  );
}
