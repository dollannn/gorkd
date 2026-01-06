import { browser } from '$app/environment'

type Theme = 'light' | 'dark'

const STORAGE_KEY = 'theme'

function getInitialTheme(): Theme {
	if (!browser) return 'light'

	const stored = localStorage.getItem(STORAGE_KEY)
	if (stored === 'light' || stored === 'dark') {
		return stored
	}

	if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
		return 'dark'
	}

	return 'light'
}

function createThemeStore() {
	let theme = $state<Theme>(getInitialTheme())

	$effect(() => {
		if (!browser) return

		localStorage.setItem(STORAGE_KEY, theme)

		if (theme === 'dark') {
			document.documentElement.classList.add('dark')
		} else {
			document.documentElement.classList.remove('dark')
		}
	})

	return {
		get value() {
			return theme
		},
		toggle() {
			theme = theme === 'light' ? 'dark' : 'light'
		},
		set(newTheme: Theme) {
			theme = newTheme
		},
	}
}

export const themeStore = createThemeStore()
