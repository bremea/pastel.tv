export interface ApiError {
	error: true;
	message: string;
}

export type ApiResult<T> =
	| ({
			error: false | undefined;
	  } & T)
	| ApiError;

export interface EmailInfo {
	login_method: LoginMethod;
	new_account: boolean;
}

export enum LoginMethod {
	PASSWORD,
	GOOGLE
};
