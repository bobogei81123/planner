import * as CheckboxPrimitive from '@radix-ui/react-checkbox';
import { Check } from 'lucide-react';
import * as React from 'react';

import { cn } from '@/lib/utils';

const TodoCheckbox = React.forwardRef<
  React.ElementRef<typeof CheckboxPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof CheckboxPrimitive.Root> & { className?: string }
>(({ className, ...props }, ref) => (
  <CheckboxPrimitive.Root
    ref={ref}
    className={cn(
      'peer h-full w-full shrink-0 rounded-full border border-primary/40 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-green-600 data-[state=checked]:text-primary-foreground data-[state=checked]:border-green-600',
      className,
    )}
    {...props}
  >
    <CheckboxPrimitive.Indicator className="flex items-center justify-center text-current">
      <Check className="h-[80%] w-[80%]" />
    </CheckboxPrimitive.Indicator>
  </CheckboxPrimitive.Root>
));

TodoCheckbox.displayName = 'TodoCheckbox';

// TODO: Fix prevent default issue
// interface TodoCheckboxProps extends React.HTMLAttributes<HTMLButtonElement> {
//   checked: boolean;
// }
//
// function TodoCheckbox({ checked, ...props }: TodoCheckboxProps) {
//   return (
//     <button
//       className={cn(
//         'peer h-full w-full shrink-0 rounded-full border border-primary/40 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-green-600 data-[state=checked]:text-primary-foreground data-[state=checked]:border-green-600',
//       )}
//       {...props}
//     >
//       {checked && (
//         <span className="flex items-center justify-center text-current">
//           <Check className="h-[80%] w-[80%]" />
//         </span>
//       )}
//     </button>
//   );
// }

function CostSquare({ cost }: { cost: number }) {
  return (
    <div className="w-full h-full bg-muted rounded-sm flex items-center justify-center">
      <span className="text-xl">{cost}</span>
    </div>
  );
}

export { TodoCheckbox, CostSquare };
