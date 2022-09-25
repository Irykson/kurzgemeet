import classNames from "classnames";

interface ButtonProps extends React.ComponentProps<"button"> {
  color?: 'primary' | 'secondary' | 'accent';
  variant?: 'outlined'
}

export function Button(props: ButtonProps) {
  const { className, children, ...otherProps } = props;
  const btnClass = classNames({
    'btn-primary': props.color === 'primary',
    'btn-accent': props.color === 'accent',
    'btn-secondary': props.color === 'secondary',
    'btn-outline': props.variant === 'outlined',
  })

  return (
    <button className={`btn ${btnClass} ${props.className}`} {...otherProps}>{children}</button>
  );
}
