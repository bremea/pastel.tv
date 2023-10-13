/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'bright-pink': {
					DEFAULT: '#ff496a',
					100: '#41000c',
					200: '#830018',
					300: '#c40024',
					400: '#ff0634',
					500: '#ff496a',
					600: '#ff6c87',
					700: '#ff91a5',
					800: '#ffb6c3',
					900: '#ffdae1'
				},
				'princeton-orange': {
					DEFAULT: '#ff9633',
					100: '#3d1e00',
					200: '#7a3b00',
					300: '#b85900',
					400: '#f57600',
					500: '#ff9633',
					600: '#ffab5c',
					700: '#ffc085',
					800: '#ffd5ad',
					900: '#ffead6'
				},
				jonquil: {
					DEFAULT: '#ffcf23',
					100: '#3a2e00',
					200: '#745b00',
					300: '#ae8900',
					400: '#e9b600',
					500: '#ffcf23',
					600: '#ffd950',
					700: '#ffe27b',
					800: '#ffeca7',
					900: '#fff5d3'
				},
				'spring-green': {
					DEFAULT: '#5affa7',
					100: '#004520',
					200: '#008b41',
					300: '#00d061',
					400: '#16ff83',
					500: '#5affa7',
					600: '#7cffb9',
					700: '#9dffcb',
					800: '#beffdc',
					900: '#deffee'
				},
				'vivid-sky-blue': {
					DEFAULT: '#00d3fd',
					100: '#002a33',
					200: '#005566',
					300: '#007f99',
					400: '#00aacc',
					500: '#00d3fd',
					600: '#33ddff',
					700: '#66e5ff',
					800: '#99eeff',
					900: '#ccf6ff'
				},
				phlox: {
					DEFAULT: '#c744f3',
					100: '#2d043a',
					200: '#5a0775',
					300: '#860baf',
					400: '#b30fea',
					500: '#c744f3',
					600: '#d269f5',
					700: '#de8ff8',
					800: '#e9b4fa',
					900: '#f4dafd'
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
				}
			}
		}
	},
	plugins: []
};
