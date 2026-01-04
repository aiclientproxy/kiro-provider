/**
 * FingerprintPanel 组件
 * 显示设备指纹信息
 */

import {
  Fingerprint,
  ChevronUp,
  Copy,
  Check,
  cn,
} from "@proxycast/plugin-components";
import type { FingerprintPanelProps } from "./types";

export function FingerprintPanel({
  expanded,
  loading,
  info,
  onClose,
  onCopyMachineId,
  copied,
}: FingerprintPanelProps) {
  if (!expanded) return null;

  return (
    <div className="mx-4 mb-4 p-4 rounded-lg bg-indigo-50 dark:bg-indigo-950/30 border border-indigo-200 dark:border-indigo-800">
      <div className="flex items-center justify-between mb-3">
        <span className="text-sm font-medium text-indigo-700 dark:text-indigo-300 flex items-center gap-2">
          <Fingerprint className="h-4 w-4" />
          设备指纹
        </span>
        <button
          onClick={onClose}
          className="text-indigo-500 hover:text-indigo-700 dark:hover:text-indigo-300"
        >
          <ChevronUp className="h-4 w-4" />
        </button>
      </div>

      {loading ? (
        <div className="flex items-center gap-2 text-sm text-indigo-600 dark:text-indigo-400">
          <div className="animate-spin h-4 w-4 border-2 border-current border-t-transparent rounded-full" />
          加载中...
        </div>
      ) : info ? (
        <div className="space-y-3">
          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">Machine ID:</span>
            <code className="text-sm font-mono bg-white dark:bg-gray-800 px-2 py-1 rounded border">
              {info.machine_id_short}...
            </code>
            <button
              onClick={onCopyMachineId}
              className="p-1.5 rounded hover:bg-indigo-100 dark:hover:bg-indigo-900/50 transition-colors"
              title="复制完整 Machine ID"
            >
              {copied ? (
                <Check className="h-4 w-4 text-green-500" />
              ) : (
                <Copy className="h-4 w-4 text-muted-foreground" />
              )}
            </button>
          </div>
          <div className="flex items-center gap-6 text-sm">
            <span className="flex items-center gap-2">
              <span className="text-muted-foreground">来源:</span>
              <span
                className={cn(
                  "px-2 py-0.5 rounded font-medium",
                  info.source === "profileArn"
                    ? "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400"
                    : info.source === "clientId"
                      ? "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400"
                      : "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400"
                )}
              >
                {info.source}
              </span>
            </span>
            <span className="flex items-center gap-2">
              <span className="text-muted-foreground">认证:</span>
              <span
                className={cn(
                  "px-2 py-0.5 rounded font-medium",
                  info.auth_method.toLowerCase() === "idc"
                    ? "bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400"
                    : "bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400"
                )}
              >
                {info.auth_method}
              </span>
            </span>
          </div>
        </div>
      ) : (
        <div className="text-sm text-muted-foreground">无法获取指纹信息</div>
      )}
    </div>
  );
}
