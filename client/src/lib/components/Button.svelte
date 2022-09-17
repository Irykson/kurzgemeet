<script lang="ts">
  export let color: 'primary' | 'secondary' | 'accent' | undefined = undefined;
  export let variant: 'outlined' | undefined = undefined;
  export let className = '';
  export { className as class };
</script>

<button
  {...$$restProps}
  on:click
  class="btn {className}"
  class:btn-primary={color === 'primary'}
  class:btn-secondary={color === 'secondary'}
  class:btn-accent={color === 'accent'}
  class:btn-outline={variant === 'outlined'}
>
  <slot />
</button>
