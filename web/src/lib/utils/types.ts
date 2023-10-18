export interface ApiError {
	error: boolean;
	message: string;
}

export interface EmailInfo {
	loginMethod: LoginMethod;
}

export type LoginMethod = 'PASS' | 'GOOGLE'