import { UserConfigExport, ConfigEnv, loadEnv } from "vite";
import vue from '@vitejs/plugin-vue'
import { processExpression } from '@vue/compiler-core'

// 全局配置文件 全局css样式、或者api
// https://vitejs.dev/config/
//
//
// export default defineConfig({
//   plugins: [vue()],
//   server: {
//     // 服务器主机名，如果允许外部访问，可设置为"0.0.0.0"
//     host: '0.0.0.0',
//     port: 8888,
//     // boolean | string 启动项目时自动在浏览器打开应用程序；如果为string，比如"/index.html"，会打开http://localhost:3000/index.html
//     open: false,
//   },
// })

export default ({ command, mode }: ConfigEnv): UserConfigExport => {
  const root = process.cwd()

  const env = loadEnv(mode, root) as unknown as ImportMetaEnv
}
