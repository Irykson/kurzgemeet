<script lang="ts">
  export let variant: 'primary' | 'secondary' | undefined = undefined;
  export let className = '';
  export { className as class };
  export let value = '';
</script>

<input
  bind:value
  class="input input-bordered {className}"
  class:input-primary={variant === 'primary'}
  class:input-secondary={variant === 'secondary'}
  {...$$restProps}
/>
