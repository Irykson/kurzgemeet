export function getOrInitInLocalStorage<T>(key: string, defaultValue: T): T {
  const existing = localStorage.getItem(key);
  if (existing === null) {
    localStorage.setItem(key, JSON.stringify(defaultValue));
    return defaultValue;
  } else {
    return JSON.parse(existing);
  }
}

export function pushToLocalStorage<T>(key: string, value: T) {
  const existing = getOrInitInLocalStorage<T[]>(key, []);

  existing.push(value);

  localStorage.setItem(key, JSON.stringify(existing));
}

export function getRandomPassword(): string {
  return (Math.random() + 1).toString(36).substring(2);
}

