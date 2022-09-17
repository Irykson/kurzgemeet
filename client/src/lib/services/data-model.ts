export interface User {
  id: number;
  name: string;
}

export interface PasswordUser extends User {
  password: string;
}

export interface EmailUser extends User {
  email: string;
  password: string; // how to encrypt?
}

export interface DateVote {
  id: number;
  date: Date;
  users: User[];
}

export interface PlaceVote {
  id: number;
  place: string;
  users: User[];
}

export interface Meetup {
  id: number; // will
  password: string;
  name: string;
  description: string;
  date: Date;
  place: string;
  organizor: User;
  attendees: User[];
  dateVotes: DateVote[];
  placeVotes: PlaceVote[];
}
