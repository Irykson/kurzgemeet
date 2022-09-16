<script lang="ts">
	export let variant: 'primary' | 'secondary' | undefined = undefined;
	export let className = '';
	export { className as class };
</script>

<input
	class="input input-bordered {className}"
	class:input-primary={variant === 'primary'}
	class:input-secondary={variant === 'secondary'}
	{...$$restProps}
/>
