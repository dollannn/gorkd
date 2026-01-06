<script lang="ts">
	import { Search } from 'lucide-svelte'
	import { Button, Input, Card, Spinner } from '$lib/components'

	let query = $state('')
</script>

<div class="space-y-8">
	<section class="text-center">
		<h1 class="mb-2 text-3xl font-bold" style="color: var(--color-text);">
			Research anything
		</h1>
		<p style="color: var(--color-text-muted);">
			Ask a question, get a cited answer with sources.
		</p>
	</section>

	<Card>
		<div class="flex flex-col gap-4 sm:flex-row">
			<div class="flex-1">
				<Input
					placeholder="What caused the 2024 CrowdStrike outage?"
					bind:value={query}
				/>
			</div>
			<Button variant="primary">
				<Search class="h-4 w-4" />
				Research
			</Button>
		</div>
	</Card>

	<section class="space-y-4">
		<h2 class="text-lg font-semibold" style="color: var(--color-text);">
			Component Demo
		</h2>

		<Card title="Buttons">
			<div class="flex flex-wrap gap-3">
				<Button variant="primary">Primary</Button>
				<Button variant="secondary">Secondary</Button>
				<Button variant="ghost">Ghost</Button>
				<Button variant="primary" loading>Loading</Button>
				<Button variant="primary" disabled>Disabled</Button>
			</div>
		</Card>

		<Card title="Inputs">
			<div class="grid gap-4 sm:grid-cols-2">
				<Input label="Normal Input" placeholder="Type something..." />
				<Input label="With Error" error="This field is required" />
			</div>
		</Card>

		<Card title="Loading States">
			<div class="flex items-center gap-4">
				<Spinner size="sm" />
				<Spinner size="md" />
				<Spinner size="lg" />
			</div>
		</Card>
	</section>
</div>
