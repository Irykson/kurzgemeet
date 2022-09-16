<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Input from '$lib/components/Input.svelte';
</script>

<div class="hero min-h-screen">
	<div class="hero-content text-center">
		<div class="flex flex-col gap-5">
			<span class="text-4xl font-bold text-primary">Join a meetup</span>
			<div class="flex gap-2">
				<Input
					type="text"
					variant="primary"
					placeholder="Invite Password"
					maxlength="10"
					minlength="10"
				/>
				<Button color="primary">Go</Button>
			</div>
			<Button color="secondary" variant="outlined" class="normal-case">
				...or organize your own!
			</Button>
		</div>
	</div>
</div>
