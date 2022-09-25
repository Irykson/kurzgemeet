<script lang="ts">
  export let variant: 'primary' | 'secondary' | undefined = undefined;
  export let className = '';
  export { className as class };
  export let value = '';
  export let label = '';
</script>

<div>
  {#if label?.length > 0}
    <div class="label">
      <span class="label-text">{label}</span>
    </div>
  {/if}
  <input
    bind:value
    class="input input-bordered {className}"
    class:input-primary={variant === 'primary'}
    class:input-secondary={variant === 'secondary'}
    {...$$restProps}
  />

  {#if $$restProps.maxlength || $$restProps.minlength !== undefined}
    <div class="label">
      <span
        class="label-text-alt"
        class:text-success={value.length === +$$restProps.minlength}
      >
        {value.length} / {$$restProps.minlength}
      </span>
    </div>
  {/if}
</div>
