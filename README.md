# Work in progress

**This is currently just a project for personal learning of Rust and modern web frameworks and coneption is far from done or documented**

## 1 What?

This project provides a planning tool for meetups with minimal hurdles for attendees (so possibly even without registration of an actual user account)

## 2 How?

- To make it easy for new users to sign up for an event, they should be able to sign-up for a meetup with a unique password, instead of an account linked to an email.
  This password will be generated automatically after the user signed-up and will be used to log in later, e.g. for changing there vote on time / place or to cancel there registration.
- To minimize "fake-sign-ups", the meetup itself cannot be searched for but only be found by entering a unique meetup-password.

### 2.1 What is a meetup-password?

For now a meetup-password technically really just some text, so basically a password like it's commonly known and it will be used as the only identifier.
To make inviting to a meetup easier, instead of a password, a short combination of visual symbols, like emojis, could be used instead.

_tbd_
