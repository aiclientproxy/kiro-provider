/**
 * UsagePanel 组件
 * 显示 Kiro 用量信息
 */

import {
  BarChart3,
  ChevronUp,
  AlertTriangle,
  Zap,
  cn,
  type UsageInfo,
} from "@proxycast/plugin-components";
import type { UsagePanelProps } from "./types";

/**
 * 格式化数字
 */
function formatNumber(num: number): string {
  if (num >= 1000000) {
    return `${(num / 1000000).toFixed(1)}M`;
  }
  if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}K`;
  }
  return num.toFixed(1);
}

/**
 * 用量显示子组件
 */
function UsageDisplay({ usage, loading }: { usage: UsageInfo; loading?: boolean }) {
  if (loading) {
    return (
      <div className="rounded-lg border p-4 animate-pulse">
        <div className="h-4 bg-muted rounded w-1/3 mb-3" />
        <div className="grid grid-cols-3 gap-4">
          <div className="h-12 bg-muted rounded" />
          <div className="h-12 bg-muted rounded" />
          <div className="h-12 bg-muted rounded" />
        </div>
      </div>
    );
  }

  const usagePercent =
    usage.usageLimit > 0
      ? Math.round((usage.currentUsage / usage.usageLimit) * 100)
      : 0;

  return (
    <div
      className={cn(
        "rounded-lg border p-4",
        usage.isLowBalance
          ? "border-amber-300 bg-amber-50/50 dark:border-amber-700 dark:bg-amber-950/30"
          : "border-border bg-card"
      )}
    >
      {/* 标题和警告 */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <Zap className="h-4 w-4 text-primary" />
          <span className="font-medium text-sm">
            {usage.subscriptionTitle || "用量信息"}
          </span>
        </div>
        {usage.isLowBalance && (
          <div className="flex items-center gap-1 text-amber-600 dark:text-amber-400">
            <AlertTriangle className="h-4 w-4" />
            <span className="text-xs font-medium">余额不足</span>
          </div>
        )}
      </div>

      {/* 进度条 */}
      <div className="mb-4">
        <div className="h-2 bg-muted rounded-full overflow-hidden">
          <div
            className={cn(
              "h-full transition-all",
              usage.isLowBalance
                ? "bg-amber-500"
                : usagePercent > 50
                  ? "bg-blue-500"
                  : "bg-green-500"
            )}
            style={{ width: `${Math.min(usagePercent, 100)}%` }}
          />
        </div>
        <div className="flex justify-between mt-1 text-xs text-muted-foreground">
          <span>已使用 {usagePercent}%</span>
          <span>剩余 {100 - usagePercent}%</span>
        </div>
      </div>

      {/* 数据统计 */}
      <div className="grid grid-cols-3 gap-3">
        <div className="text-center p-2 rounded-lg bg-muted/50">
          <div className="text-xs text-muted-foreground mb-1">总额度</div>
          <div className="font-semibold text-sm">{formatNumber(usage.usageLimit)}</div>
        </div>
        <div className="text-center p-2 rounded-lg bg-muted/50">
          <div className="text-xs text-muted-foreground mb-1">已使用</div>
          <div className="font-semibold text-sm">{formatNumber(usage.currentUsage)}</div>
        </div>
        <div
          className={cn(
            "text-center p-2 rounded-lg",
            usage.isLowBalance ? "bg-amber-100 dark:bg-amber-900/30" : "bg-muted/50"
          )}
        >
          <div
            className={cn(
              "text-xs mb-1",
              usage.isLowBalance
                ? "text-amber-600 dark:text-amber-400"
                : "text-muted-foreground"
            )}
          >
            余额
          </div>
          <div
            className={cn(
              "font-semibold text-sm",
              usage.isLowBalance && "text-amber-600 dark:text-amber-400"
            )}
          >
            {formatNumber(usage.balance)}
          </div>
        </div>
      </div>
    </div>
  );
}

export function UsagePanel({
  expanded,
  loading,
  info,
  error,
  onClose,
}: UsagePanelProps) {
  if (!expanded) return null;

  return (
    <div className="mx-4 mb-4 p-4 rounded-lg bg-cyan-50 dark:bg-cyan-950/30 border border-cyan-200 dark:border-cyan-800">
      <div className="flex items-center justify-between mb-3">
        <span className="text-sm font-medium text-cyan-700 dark:text-cyan-300 flex items-center gap-2">
          <BarChart3 className="h-4 w-4" />
          Kiro 用量
        </span>
        <button
          onClick={onClose}
          className="text-cyan-500 hover:text-cyan-700 dark:hover:text-cyan-300"
        >
          <ChevronUp className="h-4 w-4" />
        </button>
      </div>

      {error ? (
        <div className="rounded-lg bg-red-100 p-3 text-sm text-red-700 dark:bg-red-900/30 dark:text-red-300">
          {error}
        </div>
      ) : info ? (
        <UsageDisplay usage={info} loading={loading} />
      ) : (
        <UsageDisplay
          usage={{
            subscriptionTitle: "",
            usageLimit: 0,
            currentUsage: 0,
            balance: 0,
            isLowBalance: false,
          }}
          loading={true}
        />
      )}
    </div>
  );
}
