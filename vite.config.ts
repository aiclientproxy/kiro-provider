import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import fs from 'fs';

// 检查是否在 CI 环境或主应用组件库是否存在
const isCI = process.env.CI === 'true' || process.env.GITHUB_ACTIONS === 'true';
const proxycastComponentsPath = path.resolve(__dirname, '../proxycast/src/lib/plugin-components');
const hasLocalComponents = fs.existsSync(proxycastComponentsPath);

export default defineConfig({
  plugins: [react()],
  define: {
    // 定义 process.env.NODE_ENV，避免运行时报错
    'process.env.NODE_ENV': JSON.stringify('production'),
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      // 仅在本地开发且主应用存在时使用 alias
      // CI 环境使用类型声明文件，运行时从全局变量获取
      ...(hasLocalComponents && !isCI ? {
        '@proxycast/plugin-components': proxycastComponentsPath,
      } : {}),
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
