/**
 * CredentialCard 主组件
 * 组合所有子组件显示完整的凭证卡片
 */

import { cn } from "@proxycast/plugin-components";
import { useCredentialActions } from "../../hooks/useCredentialActions";
import { CardHeader } from "./CardHeader";
import { CardActions } from "./CardActions";
import { CardStats } from "./CardStats";
import { FingerprintPanel } from "./FingerprintPanel";
import { UsagePanel } from "./UsagePanel";
import { StatusPanel } from "./StatusPanel";
import { SwitchResultPanel } from "./SwitchResultPanel";
import type { CredentialCardProps } from "./types";

export function CredentialCard({
  credential,
  onToggle,
  onDelete,
  onReset,
  onCheckHealth,
  onRefreshToken,
  onEdit,
  deleting,
  checkingHealth,
  refreshingToken,
  isLocalActive,
  onSwitchToLocal,
}: CredentialCardProps) {
  const isHealthy = (credential.is_healthy ?? false) && !credential.is_disabled;
  const isOAuth = (credential.credential_type || "").includes("oauth");

  // 使用 hook 管理所有 Kiro 特有的状态和操作
  const {
    fingerprintInfo,
    fingerprintLoading,
    fingerprintExpanded,
    fingerprintCopied,
    usageInfo,
    usageLoading,
    usageExpanded,
    usageError,
    kiroHealthScore,
    kiroStatusLoading,
    kiroStatusExpanded,
    kiroRefreshing,
    switchingToLocal,
    switchResult,
    handleCheckFingerprint,
    handleCopyMachineId,
    handleCheckUsage,
    handleCheckKiroStatus,
    handleQuickRefresh,
    handleSwitchToLocal,
    closeFingerprintPanel,
    closeUsagePanel,
    closeStatusPanel,
  } = useCredentialActions({
    credentialUuid: credential.uuid,
    onRefreshToken,
    onSwitchToLocal,
  });

  return (
    <div
      className={cn(
        "rounded-xl border-2 transition-all hover:shadow-md",
        credential.is_disabled
          ? "border-gray-200 bg-gray-50/80 opacity-70 dark:border-gray-700 dark:bg-gray-900/60"
          : isLocalActive
            ? "border-amber-400 bg-gradient-to-r from-amber-50/80 to-white dark:border-amber-500 dark:bg-gradient-to-r dark:from-amber-950/40 dark:to-transparent"
            : isHealthy
              ? "border-green-200 bg-gradient-to-r from-green-50/80 to-white dark:border-green-800 dark:bg-gradient-to-r dark:from-green-950/40 dark:to-transparent"
              : "border-red-200 bg-gradient-to-r from-red-50/80 to-white dark:border-red-800 dark:bg-gradient-to-r dark:from-red-950/40 dark:to-transparent"
      )}
    >
      {/* 第一行：状态图标 + 名称 + 标签 + 操作按钮 */}
      <div className="flex items-center gap-4 p-4 pb-3">
        <CardHeader
          credential={credential}
          isHealthy={isHealthy}
          isLocalActive={isLocalActive}
        />

        <CardActions
          credential={credential}
          isOAuth={isOAuth}
          checkingHealth={checkingHealth}
          refreshingToken={refreshingToken || false}
          fingerprintLoading={fingerprintLoading}
          fingerprintExpanded={fingerprintExpanded}
          usageLoading={usageLoading}
          usageExpanded={usageExpanded}
          kiroStatusLoading={kiroStatusLoading}
          kiroStatusExpanded={kiroStatusExpanded}
          kiroRefreshing={kiroRefreshing}
          switchingToLocal={switchingToLocal}
          deleting={deleting}
          onToggle={onToggle}
          onEdit={onEdit}
          onCheckHealth={onCheckHealth}
          onRefreshToken={onRefreshToken}
          onCheckFingerprint={handleCheckFingerprint}
          onCheckUsage={handleCheckUsage}
          onCheckKiroStatus={handleCheckKiroStatus}
          onQuickRefresh={handleQuickRefresh}
          onSwitchToLocal={handleSwitchToLocal}
          onReset={onReset}
          onDelete={onDelete}
        />
      </div>

      {/* 第二行：统计信息 */}
      <CardStats
        credential={credential}
        isOAuth={isOAuth}
        kiroHealthScore={kiroHealthScore}
      />

      {/* 第三行：UUID */}
      <div className="px-4 py-2 border-t border-border/30">
        <p className="text-xs text-muted-foreground font-mono">
          {credential.uuid}
        </p>
      </div>

      {/* 错误信息 */}
      {credential.last_error_message && (
        <div className="mx-4 mb-3 rounded-lg bg-red-100 p-3 text-xs text-red-700 dark:bg-red-900/30 dark:text-red-300">
          {credential.last_error_message.slice(0, 150)}
          {credential.last_error_message.length > 150 && "..."}
        </div>
      )}

      {/* 切换到本地结果提示 */}
      <SwitchResultPanel result={switchResult} />

      {/* 指纹信息面板 */}
      <FingerprintPanel
        expanded={fingerprintExpanded}
        loading={fingerprintLoading}
        info={fingerprintInfo}
        onClose={closeFingerprintPanel}
        onCopyMachineId={handleCopyMachineId}
        copied={fingerprintCopied}
      />

      {/* Kiro 详细状态面板 */}
      <StatusPanel
        expanded={kiroStatusExpanded}
        loading={kiroStatusLoading}
        healthScore={kiroHealthScore}
        credential={credential}
        kiroRefreshing={kiroRefreshing}
        checkingHealth={checkingHealth}
        onClose={closeStatusPanel}
        onToggle={onToggle}
        onQuickRefresh={handleQuickRefresh}
        onCheckHealth={onCheckHealth}
      />

      {/* 用量信息面板 */}
      <UsagePanel
        expanded={usageExpanded}
        loading={usageLoading}
        info={usageInfo}
        error={usageError}
        onClose={closeUsagePanel}
      />
    </div>
  );
}
