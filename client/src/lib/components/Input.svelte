<script lang="ts">
	export let variant: 'primary' | 'secondary' | undefined = undefined;
</script>

<input
	class="input input-bordered"
	class:input-primary={variant === 'primary'}
	class:input-secondary={variant === 'secondary'}
	{...$$restProps}
/>
