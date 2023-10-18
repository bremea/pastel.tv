/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'carnation-pink': {
					DEFAULT: '#ff99c9',
					100: '#520026',
					200: '#a3004c',
					300: '#f50072',
					400: '#ff479d',
					500: '#ff99c9',
					600: '#ffadd3',
					700: '#ffc2de',
					800: '#ffd6e9',
					900: '#ffebf4'
				},
				vanilla: {
					DEFAULT: '#fbf4b1',
					100: '#514a04',
					200: '#a39309',
					300: '#f2dc0f',
					400: '#f7e860',
					500: '#fbf4b1',
					600: '#fcf6c1',
					700: '#fdf8d0',
					800: '#fdfae0',
					900: '#fefdef'
				},
				celadon: {
					DEFAULT: '#bdefd1',
					100: '#114526',
					200: '#218a4b',
					300: '#33ce71',
					400: '#78dea1',
					500: '#bdefd1',
					600: '#caf2da',
					700: '#d8f5e4',
					800: '#e5f9ed',
					900: '#f2fcf6'
				},
				'uranian-blue': {
					DEFAULT: '#a9def9',
					100: '#05364e',
					200: '#0b6c9c',
					300: '#10a2eb',
					400: '#5bc1f4',
					500: '#a9def9',
					600: '#bae5fa',
					700: '#ccebfb',
					800: '#ddf2fd',
					900: '#eef8fe'
				},
				night: {
					DEFAULT: '#141515',
					100: '#040404',
					200: '#080808',
					300: '#0c0c0c',
					400: '#101111',
					500: '#141515',
					600: '#424545',
					700: '#707575',
					800: '#9fa3a3',
					900: '#cfd1d1'
				},
			}
		}
	},
	plugins: []
};
