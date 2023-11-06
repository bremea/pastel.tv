import type { User } from '$lib/utils/types';
import { writable } from 'svelte/store';

export const accessToken = writable<string | undefined>(undefined);
export const currentUser = writable<User | undefined>(undefined);