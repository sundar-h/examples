import { UserConfigExport, ConfigEnv, loadEnv } from "vite";
import vue from '@vitejs/plugin-vue'
import path from "path";

// 全局配置文件 全局css样式、或者api
// https://vitejs.dev/config/
//
 interface ImportMetaEnv {
        VITE_APP_TITLE: string
        VITE_PORT: number;
        VITE_PROXY: string;
    } 

const setAlias = (alias: [string, string][]) => alias.map(v => {return { find: v[0], replacement: path.resolve(__dirname, v[1]) }})

export default ({ mode }: ConfigEnv): UserConfigExport => {
    const root = process.cwd()
    const env = loadEnv(mode, root) as unknown as ImportMetaEnv
    return {
      	// base: env.ENV === 'production' ? env.VITE_PUBLIC_PATH : './',
      base: "./",
        plugins: [ vue() ],            
        resolve: {
            alias: setAlias([
                ['/@', 'src'],
                ['/mock', 'mock'],
                ['/server', 'server']
            ])
        },
        server: {
            // proxy: env.VITE_PROXY ? proxy(JSON.parse(env.VITE_PROXY)) : {},
            port: env.VITE_PORT
        },
        build: {
          outDir: 'dist',
          minify: 'esbuild',
          sourcemap: false,
          chunkSizeWarningLimit: 1500,
        },
    }
}