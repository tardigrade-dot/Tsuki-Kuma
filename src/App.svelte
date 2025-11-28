<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import HelloWorld from '$lib/components/HelloWorld.svelte';
	import AddMath from '$lib/components/AddMath.svelte';
	import LLM_js from '$lib/components/LLM_js.svelte';
	import TTS from '$lib/components/TTS.svelte';
	import LLM from '$lib/components/LLM.svelte';
	import { onMount } from 'svelte';

	let currentPage = $state('home');
	let env = $state('browser')

	function goAdd( event) {
		console.log('dispatching add event');
		currentPage = 'add';
	}

	function goTTS() {
		console.log('dispatching tts event');
		currentPage = 'tts';
	}

	function goLLM_JS() {
		console.log('dispatching tts event');
		currentPage = 'llm_js';
	}

	function goLLM() {
		console.log('dispatching llm event');
		currentPage = "llm";
	}
	onMount(() => {
		const IS_TAURI_MODE_CHECK = 
		typeof window !== 'undefined' && window.__TAURI__ !== undefined;

		console.log('typeof window:', typeof window); // 应该为 'object'
    	console.log('window.__TAURI__:', window.__TAURI__); // 应该是一个对象，而不是 undefined
	});
</script>
<p>当前页面:{currentPage}</p>
<p>运行环境:{env}</p>
<main class="flex min-h-screen items-center justify-center bg-gray-50 p-4">
	<div class="w-full max-w-md">
		{#if currentPage === 'home'}
			<HelloWorld on:add={goAdd} on:tts={goTTS} on:llm={goLLM} on:llm_js={goLLM_JS}/>
		{:else if currentPage === 'add'}
			<AddMath on:back={() => (currentPage = 'home')} />
		{:else if currentPage === 'tts'}
			<TTS on:back={() => (currentPage = 'home')} />
		{:else if currentPage === 'llm_js'}
			<LLM_js on:back={() => (currentPage = 'home')} />
		{:else if currentPage === 'llm'}
			<LLM on:back={() => (currentPage = 'home')} />
		{/if}
	</div>
</main>
