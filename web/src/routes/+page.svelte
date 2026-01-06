<script lang="ts">
	import { Search } from 'lucide-svelte'
</script>

<main
	class="flex min-h-screen flex-col items-center justify-center bg-neutral-950 p-8"
>
	<div class="text-center">
		<h1 class="mb-4 text-4xl font-bold text-white">gorkd</h1>
		<p class="mb-8 text-neutral-400">Truth-seeking research bot</p>

		<div class="flex items-center gap-2 rounded-lg bg-neutral-900 px-4 py-3">
			<Search class="size-5 text-neutral-500" />
			<span class="text-neutral-500">Ask a question...</span>
		</div>
	</div>
</main>
