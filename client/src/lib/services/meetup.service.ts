import type { Meetup } from "./data-model";
import { getOrInitInLocalStorage, getRandomPassword, pushToLocalStorage } from "./util";


export async function getMeetupByPassword(password: string): Promise<Meetup | undefined> {
  const meetups = getOrInitInLocalStorage<Meetup[]>('meetups', []);
  return meetups.find(m => m.password === password);
}

export async function createMeetup(meetup: Partial<Meetup>) {
  meetup.password = getRandomPassword();
  pushToLocalStorage('meetup', meetup);
}

