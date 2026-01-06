import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vitest/config'

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'jsdom',
		setupFiles: ['./vitest.setup.ts'],
	},
	resolve: {
		conditions: ['browser'],
	},
	server: {
		proxy: {
			'/health': {
				target: 'http://localhost:4000',
				changeOrigin: true,
			},
			'/v1': {
				target: 'http://localhost:4000',
				changeOrigin: true,
			},
		},
	},
})
