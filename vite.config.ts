import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  define: {
    // 定义 process.env.NODE_ENV，避免运行时报错
    'process.env.NODE_ENV': JSON.stringify('production'),
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      // 指向主应用的组件库（开发时）
      '@proxycast/plugin-components': path.resolve(
        __dirname,
        '../proxycast/src/lib/plugin-components'
      ),
    },
  },
  build: {
    outDir: 'plugin/dist',
    lib: {
      entry: path.resolve(__dirname, 'src/index.tsx'),
      name: 'KiroProviderUI',
      formats: ['iife'],
      fileName: () => 'index.js',
    },
    rollupOptions: {
      // 这些依赖由主应用提供，不打包进插件
      external: [
        'react',
        'react-dom',
        '@proxycast/plugin-components',
      ],
      output: {
        // 从全局变量获取依赖
        globals: {
          'react': 'React',
          'react-dom': 'ReactDOM',
          '@proxycast/plugin-components': 'ProxyCastPluginComponents',
        },
        // IIFE 格式需要导出到全局变量
        name: 'KiroProviderPlugin',
        // 确保默认导出可用
        exports: 'named',
      },
    },
    cssCodeSplit: false,
  },
});
