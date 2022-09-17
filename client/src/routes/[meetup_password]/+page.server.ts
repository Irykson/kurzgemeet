import { redirect, type Load } from '@sveltejs/kit';

const correct_password = '1701';

export const load: Load = ({ params }) => {
  if (params.meetup_password === correct_password) {
    // TODO: return meetup data
    return {
      name: 'Awesome meetup',
      organizor: 'Steve',
    };
  } else {
    throw redirect(307, '/');
  }
};
