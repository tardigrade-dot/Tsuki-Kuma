<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { Input } from '$lib/components/ui/input/index';
    import { Button } from '$lib/components/ui/button/index';
    import { Card, Header, Title, Content } from '$lib/components/ui/card/index';
	// import { llm_chat } from '$lib/commands.svelte';
    import { pipeline, TextStreamer, env } from "@huggingface/transformers";

    interface ChatMessage {
        role: 'user' | 'assistant' | 'system';
        content: string;
    }
    console.log('cache dir: ', env.cacheDir);

    console.log('is browser: ', env.browser)
    const dispatch = createEventDispatcher();

    // 存储聊天记录的状态
    let chatHistory = $state<ChatMessage[]>([
        { role: 'assistant', content: '您好！我是 LLM 助手，有什么可以帮您的吗？' }
    ]);

    // 存储用户当前输入的状态
    let currentInput = $state('');

    // 存储加载状态
    let isLoading = $state(false);
   
    async function sendMessage() {
        if (!currentInput.trim() || isLoading) return;

        const userMessage = currentInput.trim();
        const promptToLLM = userMessage; 

        isLoading = true;
        chatHistory = [...chatHistory, { role: 'user', content: userMessage }];

        try {
            const generator = await pipeline(
                "text-generation",
                "onnx-community/Qwen3-0.6B-ONNX",
                { dtype: "q4f16" ,device: 'webgpu'},
            );
            const messages = [
                { role: "system", content: "You are a helpful assistant." },
                { role: "user", content: promptToLLM },
            ];

            // 一次性输出
            // const output = await generator(messages, {
            //     max_new_tokens: 512,
            //     do_sample: false,
            //     // streamer: new TextStreamer(generator.tokenizer, { skip_prompt: true, skip_special_tokens: true }),
            //     });
            // const responseText = output[0].generated_text.at(-1).content;
            
            // chatHistory = [...chatHistory, { role: 'assistant', content: responseText }];


            // 流式输出
            let responseText = "";

            // 创建 streamer，并定义 token 回调
            const streamer = new TextStreamer(generator.tokenizer, {
                skip_prompt: true,
                skip_special_tokens: true,
                callback_function: (token) => {
                    // 每收到一个 token，就更新 responseText
                    responseText += token;
                    // 实时更新 chatHistory 或单独状态
                    chatHistory = [
                        ...chatHistory.filter(msg => msg.role !== "assistant-stream"),
                        { role: "assistant-stream", content: responseText }
                    ];
                }
            });

            // 传入 streamer 进行流式输出
            await generator(messages, {
                max_new_tokens: 10240,
                do_sample: false,
                streamer: streamer
            });

            // 流式生成结束后，将内容写入正式 chatHistory
            chatHistory = [
                ...chatHistory.filter(msg => msg.role !== "assistant-stream"),
                { role: "assistant", content: responseText }
            ];

            isLoading = false;
        } catch (error) {
            console.error(error);
        } finally {
            isLoading = false;
        }
    }
</script>

<Card class="w-[95vw] h-[95vh] shadow-xl backdrop-blur-sm flex flex-col">
    <Header class="pt-6">
        <Title
            class="from-indigo-500 to-pink-500 bg-clip-text text-center text-3xl font-bold text-transparent"
        >
            <p>🤖 LLM 聊天助手</p>
        </Title>
    </Header>
    
    <Content class="flex-grow overflow-y-auto p-6 space-y-4">
        {#each chatHistory as message}
            <div 
                class="flex"
                class:justify-end={message.role === 'user'}
                class:justify-start={message.role === 'assistant' || message.role === 'system'}
            >
                <div 
                    class="max-w-[80%] p-3 rounded-xl whitespace-pre-wrap"
                    class:bg-indigo-500={message.role === 'user'}
                    class:text-white={message.role === 'user'}
                    class:bg-gray-100={message.role === 'assistant'}
                    class:text-gray-800={message.role === 'assistant'}
                    class:bg-red-100={message.role === 'system'}
                    class:text-red-600={message.role === 'system'}
                >
                    <p class="font-bold text-sm mb-1">
                        {message.role === 'user' ? '您' : (message.role === 'assistant' ? '助手' : '系统')}
                    </p>
                    {message.content}
                </div>
            </div>
        {/each}
        
        {#if isLoading}
            <div class="flex justify-start">
                <div class="bg-gray-200 p-3 rounded-xl text-gray-500">
                    助手正在思考...
                </div>
            </div>
        {/if}
    </Content>

    <div class="p-6 pt-0 border-t">
        <div class="flex space-x-2">
            <Input 
                type="text" 
                bind:value={currentInput} 
                placeholder="请输入您的问题..." 
                on:keydown={(e) => {
                    if (e.key === 'Enter' && !e.shiftKey) {
                        e.preventDefault();
                        sendMessage();
                    }
                }}
                class="flex-grow"
                disabled={isLoading}
            />
            <Button 
                type="button" 
                onclick={sendMessage} 
                class="from-indigo-500 to-pink-500"
                disabled={!currentInput.trim() || isLoading}
            >
                {isLoading ? '发送中...' : '发送'}
            </Button>
        </div>
        <Button onclick={() => dispatch('back')} class="mt-4 w-full" variant="outline">
            返回
        </Button>
    </div>
    
    <p class="px-6 pb-2 text-xs text-gray-500">
        Debug: {chatHistory.length} 条消息
    </p>
</Card>