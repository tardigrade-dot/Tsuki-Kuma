import { invoke } from '@tauri-apps/api/core';

type VoiceResult = [Float32Array, number]

export const preventDefault = <T extends Event>(fn: (e: T) => void): ((e: T) => void) => {
    return (e: T) => {
        e.preventDefault();
        fn(e);
    };
};

export enum FILES {
    GREET_FILE = 'greet.txt',
    NAME_FILE = 'name.txt'
}

const API_BASE_URL = 'http://127.0.0.1:13434'; 

// 检查是否在 Tauri 运行时（即应该使用 invoke）
// const IS_TAURI_MODE = typeof window !== 'undefined' && window.__TAURI__ !== undefined;
const IS_TAURI_MODE = true

// --- 实用工具函数：用于处理 HTTP API 调用 ---

async function httpPost(endpoint: string, payload: unknown): Promise<unknown> {
    const url = `${API_BASE_URL}/${endpoint}`;
    
    // 假设 VoiceResult 是一个复杂类型，这里简化为 any
    
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            // 部署分离时可能需要认证信息
        },
        body: JSON.stringify(payload),
    });

    if (!response.ok) {
        let errorMsg = `HTTP Error ${response.status}`;
        try {
            const errorData = await response.json();
            errorMsg = errorData.message || errorMsg;
        } catch {
            // 忽略 JSON 解析失败
        }
        throw new Error(errorMsg);
    }

    return response.json();
}

export async function add(a: number, b: number): Promise<number> {
    // return await invoke<number>('add', { a, b });
    if (IS_TAURI_MODE) {
        return await invoke<number>('add', { a, b });
    } else {
        // 假设您的 HTTP API 有一个 /add 接口
        const data = await httpPost('add', { a, b });
        return data.result as number; 
    }
}

export async function llm_chat(prompt: string): Promise<string> {
    return await invoke<number>('llm_infer', { prompt });
}

export async function generate_voice(text: string): Promise<number> {
    return await invoke<VoiceResult>('generate_voice',{text: text})
}

export class GlobalState {
    private _state = $state({ name: '', greet: '' });

    get greet() {
        return this._state.greet;
    }

    set greet(value: string) {
        this._state.greet = value;
    }

    get name() {
        return this._state.name;
    }

    set name(value: string) {
        this._state.name = value;
    }

    get nlen() {
        return this.name.length;
    }

    get glen() {
        return this.greet.length;
    }

    async read(path: FILES) {
        const contentFromFile = await invoke<string>('read', { path });
        if (path === FILES.NAME_FILE) {
            this.name = contentFromFile;
        } else if (path === FILES.GREET_FILE) {
            this.greet = contentFromFile;
        }
    }

    async write(path: FILES, contents: string) {
        await invoke('write', { path, contents });
        if (path === FILES.NAME_FILE) {
            this.name = contents;
        } else if (path === FILES.GREET_FILE) {
            this.greet = contents;
        }
    }

    reset() {
        this.name = '';
        this.greet = '';
    }
}
