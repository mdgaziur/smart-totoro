import type {User} from 'firebase/auth';
import {writable} from "svelte/store";

export const AuthenticatedUser = writable<User | null>(null);