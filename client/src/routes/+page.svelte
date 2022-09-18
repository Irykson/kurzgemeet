<script lang="ts">
  import { goto } from '$app/navigation';

  import Button from '$lib/components/Button.svelte';
  import Input from '$lib/components/Input.svelte';
  let invitePassword = '';

  function navigateToMeetup() {
    goto('/' + invitePassword);
  }
</script>

<div class="hero min-h-screen">
  <div class="hero-content text-center">
    <div class="flex flex-col gap-5">
      <span class="text-4xl font-bold text-primary">Join a meetup</span>
      <form class="flex gap-2">
        <Input
          type="text"
          bind:value={invitePassword}
          variant="primary"
          placeholder="Invite Password"
          maxlength="10"
          minlength="10"
        />
        <Button
          color="primary"
          type="submit"
          on:click={() => navigateToMeetup()}>Go</Button
        >
      </form>
      <Button color="secondary" variant="outlined" class="normal-case">
        ...or organize your own!
      </Button>
    </div>
  </div>
</div>
