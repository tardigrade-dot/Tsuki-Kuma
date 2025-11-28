<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Input } from '$lib/components/ui/input/index';
	import { Button } from '$lib/components/ui/button/index';
	import { Card, Header, Title, Content } from '$lib/components/ui/card/index';
	import { generate_voice } from '$lib/commands.svelte';

	const dispatch = createEventDispatcher();

	// 状态管理
	let textInput = $state('');
	let isGenerating = $state(false);
	let error = $state<string | null>(null);

	let history = $state<VoiceRecord[]>([]);

	// 生成语音
	async function generate() {
		if (!textInput.trim()) return;

		isGenerating = true;
		error = null;

		try {
			const [wavData, sampleRate] = await generate_voice(textInput.trim());

			// 转为 Float32Array 供 Web Audio 使用
			const wav = new Float32Array(wavData);
			const duration = wav.length / sampleRate;

			const record: VoiceRecord = {
				id: crypto.randomUUID(),
				text: textInput.trim(),
				wav,
				sampleRate,
				duration,
				timestamp: new Date(),
				isPlaying: false
			};

			// 保持最多 5 条（FIFO）
			history = [record, ...history].slice(0, 5);

			textInput = '';
		} catch (err: any) {
			error = err?.message || '语音生成失败';
			console.error('TTS Error:', err);
		} finally {
			isGenerating = false;
		}
	}

	// 播放音频
	async function playAudio(record: VoiceRecord) {
		// 停止所有正在播放的
		history.forEach(r => (r.isPlaying = false));
		record.isPlaying = true;

		try {
			const audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
			const buffer = audioCtx.createBuffer(1, record.wav.length, record.sampleRate);
			buffer.copyToChannel(record.wav, 0);

			const source = audioCtx.createBufferSource();
			source.buffer = buffer;
			source.connect(audioCtx.destination);
			source.start();

			// 自动停止状态
			source.onended = () => {
				record.isPlaying = false;
			};

			// 兼容 Safari 自动播放策略：首次需用户手势触发
			await audioCtx.resume(); // 确保 resume（若之前 suspended）
		} catch (err) {
			console.error('播放失败:', err);
			record.isPlaying = false;
			error = '音频播放失败，请检查浏览器权限';
		}
	}

	// 辅助：文本缩略
	function truncateText(text: string, len = 30): string {
		return text.length > len ? text.slice(0, len) + '…' : text;
	}
</script>

<Card class="w-[95vw] h-[95vh] shadow-xl backdrop-blur-sm">
	<Header class="pt-6">
		<Title class="from-indigo-500 to-pink-500 bg-clip-text text-center text-3xl font-bold text-transparent">
			Text to Speech
		</Title>
	</Header>
	<Content class="space-y-4 p-6">
		<!-- 输入区域 -->
		<div class="space-y-2">
			<label for="tts-input" class="block text-sm font-medium text-gray-700">输入文本</label>
			<textarea
				id="tts-input"
				bind:value={textInput}
				placeholder="请输入要转换为语音的文本..."
				rows="3"
				class="w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500"
			/>
			<Button
				type="button"
				onclick={generate}
				disabled={isGenerating || !textInput.trim()}
				class="mt-2 w-full bg-gradient-to-r from-indigo-500 to-pink-500 hover:from-indigo-600 hover:to-pink-600"
			>
				{#if isGenerating}
					<span class="flex items-center">
						<svg class="mr-2 h-4 w-4 animate-spin" viewBox="0 0 24 24">
							<circle cx="12" cy="12" r="10" stroke="currentColor" fill="none" stroke-width="2" />
							<path d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l2-1.647z" fill="currentColor" />
						</svg>
						生成中...
					</span>
				{:else}
					🔊 生成语音
				{/if}
			</Button>
			{#if error}
				<p class="mt-2 text-sm text-red-500">{error}</p>
			{/if}
		</div>

		<!-- 历史记录 -->
		<div class="mt-6">
			<h3 class="mb-2 text-lg font-semibold text-gray-800">最近生成</h3>
			{#if history.length === 0}
				<p class="text-center text-gray-500 italic">暂无记录，输入文本并点击“生成语音”</p>
			{:else}
				<div class="space-y-3 max-h-96 overflow-y-auto pr-2">
					{#each history as record (record.id)}
						<div class="flex items-center justify-between rounded-lg border border-gray-200 bg-white p-3 shadow-sm">
							<div class="flex-1 min-w-0">
								<p class="truncate text-sm font-medium text-gray-900">
									{truncateText(record.text)}
								</p>
								<p class="text-xs text-gray-500">
									{record.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
								</p>
								<p class="text-xs text-gray-500">
									长度: {record.duration.toFixed(2)} 秒
								</p>
							</div>
							<Button
								type="button"
								variant="ghost"
								size="sm"
								onclick={() => playAudio(record)}
								disabled={isGenerating}
								class="ml-2 flex h-8 w-8 items-center justify-center rounded-full bg-indigo-100 text-indigo-700 hover:bg-indigo-200"
							>
								{#if record.isPlaying}
									<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
										<path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
									</svg>
								{:else}
									<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
										<path d="M5 4a2 2 0 012-2h6a2 2 0 012 2v12a2 2 0 01-2 2H7a2 2 0 01-2-2V4z" />
										<path d="M7 8a1 1 0 000 2h2a1 1 0 100-2H7z" />
									</svg>
								{/if}
							</Button>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!-- 返回按钮 -->
		<Button
			onclick={() => dispatch('back')}
			variant="outline"
			class="mt-4 w-full border-gray-300 text-gray-700 hover:bg-gray-50"
		>
			← 返回
		</Button>
	</Content>
</Card>

<style>
	/* 美化滚动条（可选） */
	div[class*="overflow-y-auto"]::-webkit-scrollbar {
		width: 6px;
	}
	div[class*="overflow-y-auto"]::-webkit-scrollbar-track {
		background: #f1f1f1;
		border-radius: 3px;
	}
	div[class*="overflow-y-auto"]::-webkit-scrollbar-thumb {
		background: #c1c1c1;
		border-radius: 3px;
	}
	div[class*="overflow-y-auto"]::-webkit-scrollbar-thumb:hover {
		background: #a8a8a8;
	}
</style>