import classNames from 'classnames';

interface InputProps extends React.ComponentProps<'input'> {
  variant?: 'primary' | 'secondary';
}

export function Input(props: InputProps) {
  const inputClass = classNames({
    'input-primary': props.variant === 'primary',
    'input-secondary': props.variant === 'secondary',
  });

  return (
    <input
      className={`input input-bordered ${inputClass} ${props.className}`}
      {...props}
    />
  );
}
