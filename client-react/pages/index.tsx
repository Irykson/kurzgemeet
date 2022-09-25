import type { NextPage } from 'next';
import { useState } from 'react';
import { Button } from '../components/ui/Button';
import { Input } from '../components/ui/Input';

const Home: NextPage = () => {
  const minPasswordLength = 10;
  let [invitePassword, setInvitePassword] = useState('');

  const navigateToMeetup = () => {
    console.log('navigateToMeetup');
  };

  return (
    <div className="hero min-h-screen">
      <div className="hero-content text-center">
        <div className="flex flex-col gap-5">
          <span className="text-4xl font-bold text-primary">Join a meetup</span>
          <form className="flex gap-2">
            <Input
              type="text"
              value={invitePassword}
              onChange={(e) => setInvitePassword(e.target.value)}
              variant="primary"
              placeholder="Invite Password"
              maxLength={minPasswordLength}
              minLength={minPasswordLength}
            />
            <Button
              color="primary"
              type="submit"
              onClick={() => navigateToMeetup()}
              disabled={invitePassword.length < minPasswordLength}
            >
              Go
            </Button>
          </form>
          <Button color="secondary" variant="outlined" className="normal-case">
            ...or organize your own!
          </Button>
        </div>
      </div>
    </div>
  );
};

export default Home;
