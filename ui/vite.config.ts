import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, '../', '')
    return {
        server: { port: 4040 },
        define: {
            __AUTH0_DOMAIN__: JSON.stringify(env.AUTH0_DOMAIN),
            __AUTH0_CLIENT_ID__: JSON.stringify(env.AUTH0_CLIENT_ID),
            __AUTH0_CALLBACK_URL__: JSON.stringify(env.AUTH0_CALLBACK_URL),
            __AUTH0_AUDIENCE__: JSON.stringify(env.AUTH0_AUDIENCE),
        },
        plugins: [react()],
    }
})
