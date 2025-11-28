<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { FILES, GlobalState, preventDefault } from '$lib/commands.svelte';
	import { Input } from '$lib/components/ui/input/index';
	import { Button } from '$lib/components/ui/button/index';
	import { Card, Header, Title, Content } from '$lib/components/ui/card/index';

	const gs = new GlobalState();
	const dispatch = createEventDispatcher();

	const handleSubmit = preventDefault(async () => {
		if (gs.nlen && gs.glen) {
			await gs.write(FILES.NAME_FILE, gs.name);
			await gs.write(FILES.GREET_FILE, gs.greet);
		}
	});

	const handleReset = async () => {
		gs.reset();
		await gs.write(FILES.NAME_FILE, '');
		await gs.write(FILES.GREET_FILE, '');
	};

	$effect(() => {
		gs.read(FILES.NAME_FILE);
		gs.read(FILES.GREET_FILE);
	});

	const greeting = $derived(gs.greet || 'Hello');
	const name = $derived(gs.name || 'World');
	const message = $derived(`${greeting}, ${name}!`);

	function goAdd() {
		console.info('dispatching add event with message:', message);
		dispatch('add', { message });
	}
</script>

<Card class="w-[420px] shadow-xl backdrop-blur-sm">
	<Header class="pt-6">
		<Title
			class="bg-linear-to-r from-indigo-500 to-pink-500 bg-clip-text text-center text-3xl font-bold text-transparent"
		>
			<p>{gs.greet || 'Hello'}, {gs.name || 'World'}!</p>
		</Title>
	</Header>
	<Content class="p-6">
		<form on:submit={handleSubmit} class="space-y-3">
			<div>
				<!-- svelte-ignore a11y_label_has_associated_control -->
				<label class="text-sm font-medium">Greeting Phrase</label>
				<Input
					bind:value={gs.greet}
					class="mt-1 border border-indigo-500 focus-visible:ring-2 focus-visible:ring-purple-500"
				/>
			</div>
			<div>
				<label for="name-input" class="text-sm font-medium">Your Name</label>
				<Input
					autocomplete="off"
					autocorrect="off"
					id="name-input"
					type="text"
					placeholder="Enter your name"
					bind:value={gs.name}
					class="mt-1 border border-indigo-500 focus-visible:ring-2 focus-visible:ring-purple-500"
				/>
			</div>
			<div class="flex space-x-4">
				<Button type="submit" class="flex-1 bg-linear-to-r from-indigo-500 to-pink-500">
					Save
				</Button>
				<Button type="button" variant="outline" onclick={handleReset} class="flex-1 ">Reset</Button>
			</div>
		</form>

		<div class="mt-6 flex space-x-4">
			<Button type="button" onclick={goAdd}>Add Math</Button>
			<Button type="button" onclick={() => dispatch('tts')}>TTS</Button>
			<Button type="button" onclick={() => dispatch('llm')}>LLM</Button>
			<Button type="button" onclick={() => dispatch('llm_js')}>LLM_js</Button>
		</div>
	</Content>
</Card>
