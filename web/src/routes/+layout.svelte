<script lang="ts">
	import '../app.css'
	import favicon from '$lib/assets/favicon.svg'
	import { Logo, ThemeToggle } from '$lib/components'
	import '$lib/stores/theme.svelte'

	let { children } = $props()

	function handleKeydown(event: KeyboardEvent) {
		// Focus search input when "/" is pressed (unless already in an input)
		if (
			event.key === '/' &&
			!['INPUT', 'TEXTAREA'].includes((event.target as HTMLElement)?.tagName)
		) {
			event.preventDefault()
			const searchInput = document.querySelector<HTMLInputElement>(
				'[data-search-input]'
			)
			searchInput?.focus()
		}
	}
</script>

<svelte:head>
	<title>gorkd - Research Assistant</title>
	<meta name="description" content="Ask a question, get a cited answer with sources. Truth-seeking research bot." />
	<meta property="og:title" content="gorkd" />
	<meta property="og:description" content="Truth-seeking research bot. Ask a question, get a cited answer." />
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="gorkd" />
	<meta name="twitter:description" content="Truth-seeking research bot. Ask a question, get a cited answer." />
	<link rel="icon" href={favicon} />
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<a
	href="#main-content"
	class="sr-only focus:not-sr-only focus:absolute focus:left-4 focus:top-4 focus:z-50 focus:rounded-md focus:px-4 focus:py-2"
	style="background-color: var(--color-accent); color: white;"
>
	Skip to main content
</a>

<div
	class="flex min-h-screen flex-col"
	style="background-color: var(--color-bg);"
>
	<header
		class="sticky top-0 z-10 border-b"
		style="background-color: var(--color-bg); border-color: var(--color-border);"
	>
		<div class="mx-auto flex h-14 max-w-4xl items-center justify-between px-4">
			<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
			<a href="/" class="hover:opacity-80 transition-opacity">
				<Logo size="md" />
			</a>
			<ThemeToggle />
		</div>
	</header>

	<main id="main-content" class="flex-1">
		<div class="mx-auto max-w-4xl px-4 py-8">
			{@render children()}
		</div>
	</main>

	<footer class="border-t py-6" style="border-color: var(--color-border);">
		<div class="mx-auto max-w-4xl px-4 text-center">
			<span class="text-sm" style="color: var(--color-text-muted);">
				gorkd &middot; Truth-seeking research bot
			</span>
		</div>
	</footer>
</div>
