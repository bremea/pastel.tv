export interface ApiError {
	error: true;
	message: string;
}

export type ApiResult<T> =
	| ({
			error: false | undefined;
	  } & T)
	| ApiError;

export interface User {
	name: string;
	uuid: string;
	email: string;
	flags: number;
}

export enum LoginMethod {
	PASSWORD,
	GOOGLE
}
