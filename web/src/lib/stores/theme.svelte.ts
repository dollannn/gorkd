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

function applyTheme(theme: Theme): void {
	if (!browser) return

	localStorage.setItem(STORAGE_KEY, theme)

	if (theme === 'dark') {
		document.documentElement.classList.add('dark')
	} else {
		document.documentElement.classList.remove('dark')
	}
}

function createThemeStore() {
	let theme = $state<Theme>(getInitialTheme())

	if (browser) {
		applyTheme(theme)
	}

	return {
		get value() {
			return theme
		},
		toggle() {
			theme = theme === 'light' ? 'dark' : 'light'
			applyTheme(theme)
		},
		set(newTheme: Theme) {
			theme = newTheme
			applyTheme(theme)
		},
	}
}

export const themeStore = createThemeStore()
