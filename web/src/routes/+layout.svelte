<script lang="ts">
	import '../app.css'
	import favicon from '$lib/assets/favicon.svg'
	import { Logo, ThemeToggle } from '$lib/components'
	import '$lib/stores/theme.svelte'

	let { children } = $props()
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

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

	<main class="flex-1">
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
