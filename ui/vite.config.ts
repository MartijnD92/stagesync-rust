import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, '../', '')
    return {
        server: { port: 4040 },
        define: {
            __AUTH0_DOMAIN__: JSON.stringify(env.AUTH0_DOMAIN),
            // TODO: other variables I need
        },
        plugins: [react()],
    }
})
