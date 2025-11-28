<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { Input } from '$lib/components/ui/input/index';
    import { Button } from '$lib/components/ui/button/index';
    import { Card, Header, Title, Content } from '$lib/components/ui/card/index';
	import { llm_chat } from '$lib/commands.svelte';

    // 定义聊天消息的接口或类型
    interface ChatMessage {
        role: 'user' | 'assistant' | 'system';
        content: string;
    }

    const dispatch = createEventDispatcher();

    // 存储聊天记录的状态
    let chatHistory = $state<ChatMessage[]>([
        { role: 'assistant', content: '您好！我是 LLM 助手，有什么可以帮您的吗？' }
    ]);

    // 存储用户当前输入的状态
    let currentInput = $state('');

    // 存储加载状态
    let isLoading = $state(false);

    /**
     * 发送用户消息并调用 LLM 进行回复
     */
    async function sendMessage() {
        if (!currentInput.trim() || isLoading) return;

        const userMessage = currentInput.trim();
        isLoading = true;
        currentInput = ''; // 清空输入框

        // 1. 添加用户消息到记录
        chatHistory = [...chatHistory, { role: 'user', content: userMessage }];

        // 2. 构造 LLM Prompt (这里需要根据您的 LLM 命令要求进行格式化)
        // 注意：如果您的 llm_chat 已经处理了格式化，这里可以直接传入 userMessage
        const promptToLLM = userMessage; 

        // 3. 调用 Tauri Command
        try {
            // llm_chat 应该返回 Result<String, String>
            const responseText: string = await llm_chat(promptToLLM);
            
            // 4. 添加助手的回复
            chatHistory = [...chatHistory, { role: 'assistant', content: responseText }];

        } catch (error) {
            console.error('LLM Inference Error:', error);
            // 添加错误提示到聊天记录
            chatHistory = [...chatHistory, { 
                role: 'system', 
                content: `错误：调用模型失败。详情: ${error}` 
            }];
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