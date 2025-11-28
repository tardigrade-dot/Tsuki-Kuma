<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Input } from '$lib/components/ui/input/index';
	import { Button } from '$lib/components/ui/button/index';
	import { Card, Header, Title, Content } from '$lib/components/ui/card/index';
	import { add } from '$lib/commands.svelte';

	const dispatch = createEventDispatcher();

	let a: number | '' = '';
	let b: number | '' = '';
	let result = $state(null as number | null);

	async function calculate() {
		try {
			result = await add(a, b);
			console.log('result:', result);
		} catch (error) {
			console.error('Error:', error);
			result = null;
		}
	}
</script>

<Card class="w-[420px] shadow-xl backdrop-blur-sm">
	<Header class="pt-6">
		<Title
			class=" from-indigo-500 to-pink-500 bg-clip-text text-center text-3xl font-bold text-transparent"
		>
			<p>Add Math</p>
		</Title>
	</Header>
	<Content class="space-y-4 p-6">
		<div>
			<Input type="number" bind:value={a} placeholder="Number A" />
			<Input type="number" bind:value={b} placeholder="Number B" />
			<Button type="button" onclick={calculate} class="w-full  from-indigo-500 to-pink-500"
				>Calculate</Button
			>
			{#if true}
				<p class="text-center text-xl font-bold">Result: {result}</p>
			{/if}
		</div>
		<Button onclick={() => dispatch('back')} class="mt-4 w-full">Back</Button>
	</Content>
	<p>Debug: {result}</p>
</Card>
