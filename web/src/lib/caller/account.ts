import type { EmailInfo } from "$lib/utils/types";

export async function checkEmail(email: string): Promise<EmailInfo> {
	console.log(email)
	return {
		loginMethod: 'GOOGLE'
	}
}