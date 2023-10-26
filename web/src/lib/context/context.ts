import type { User } from '$lib/utils/types';
import { writable } from 'svelte/store';

export const currentUser = writable<User | undefined>(undefined)