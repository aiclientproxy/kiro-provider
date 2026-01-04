/**
 * useCredentialActions Hook
 * 管理凭证卡片的所有状态和操作
 */

import { useState, useCallback } from "react";
import {
  getKiroCredentialFingerprint,
  switchKiroToLocal,
  kiroCredentialApi,
  usageApi,
  type KiroFingerprintInfo,
  type UsageInfo,
} from "@proxycast/plugin-components";
import type { SwitchToLocalResult } from "../components/CredentialCard/types";

interface UseCredentialActionsProps {
  credentialUuid: string;
  onRefreshToken?: () => void;
  onSwitchToLocal?: () => void;
}

interface UseCredentialActionsReturn {
  // 指纹状态
  fingerprintInfo: KiroFingerprintInfo | null;
  fingerprintLoading: boolean;
  fingerprintExpanded: boolean;
  fingerprintCopied: boolean;
  // 用量状态
  usageInfo: UsageInfo | null;
  usageLoading: boolean;
  usageExpanded: boolean;
  usageError: string | null;
  // Kiro 状态
  kiroHealthScore: number | null;
  kiroStatusLoading: boolean;
  kiroStatusExpanded: boolean;
  kiroRefreshing: boolean;
  // 切换到本地状态
  switchingToLocal: boolean;
  switchResult: SwitchToLocalResult | null;
  // 操作方法
  handleCheckFingerprint: () => Promise<void>;
  handleCopyMachineId: () => Promise<void>;
  handleCheckUsage: () => Promise<void>;
  handleCheckKiroStatus: () => Promise<void>;
  handleQuickRefresh: () => Promise<void>;
  handleSwitchToLocal: () => Promise<void>;
  // 面板关闭方法
  closeFingerprintPanel: () => void;
  closeUsagePanel: () => void;
  closeStatusPanel: () => void;
}

export function useCredentialActions({
  credentialUuid,
  onRefreshToken,
  onSwitchToLocal,
}: UseCredentialActionsProps): UseCredentialActionsReturn {
  // 指纹状态
  const [fingerprintInfo, setFingerprintInfo] = useState<KiroFingerprintInfo | null>(null);
  const [fingerprintLoading, setFingerprintLoading] = useState(false);
  const [fingerprintExpanded, setFingerprintExpanded] = useState(false);
  const [fingerprintCopied, setFingerprintCopied] = useState(false);

  // 用量状态
  const [usageInfo, setUsageInfo] = useState<UsageInfo | null>(null);
  const [usageLoading, setUsageLoading] = useState(false);
  const [usageExpanded, setUsageExpanded] = useState(false);
  const [usageError, setUsageError] = useState<string | null>(null);

  // Kiro 状态
  const [kiroHealthScore, setKiroHealthScore] = useState<number | null>(null);
  const [kiroStatusLoading, setKiroStatusLoading] = useState(false);
  const [kiroStatusExpanded, setKiroStatusExpanded] = useState(false);
  const [kiroRefreshing, setKiroRefreshing] = useState(false);

  // 切换到本地状态
  const [switchingToLocal, setSwitchingToLocal] = useState(false);
  const [switchResult, setSwitchResult] = useState<SwitchToLocalResult | null>(null);

  // 查询指纹信息
  const handleCheckFingerprint = useCallback(async () => {
    if (fingerprintExpanded && fingerprintInfo) {
      setFingerprintExpanded(false);
      return;
    }

    setFingerprintExpanded(true);
    setFingerprintLoading(true);

    try {
      const info = await getKiroCredentialFingerprint(credentialUuid);
      setFingerprintInfo(info);
    } catch (e) {
      console.error("获取指纹信息失败:", e);
    } finally {
      setFingerprintLoading(false);
    }
  }, [credentialUuid, fingerprintExpanded, fingerprintInfo]);

  // 复制 Machine ID
  const handleCopyMachineId = useCallback(async () => {
    if (!fingerprintInfo) return;
    try {
      await navigator.clipboard.writeText(fingerprintInfo.machine_id);
      setFingerprintCopied(true);
      setTimeout(() => setFingerprintCopied(false), 2000);
    } catch (e) {
      console.error("复制失败:", e);
    }
  }, [fingerprintInfo]);

  // 查询用量
  const handleCheckUsage = useCallback(async () => {
    if (usageExpanded && usageInfo) {
      setUsageExpanded(false);
      return;
    }

    setUsageExpanded(true);
    setUsageLoading(true);
    setUsageError(null);

    try {
      const info = await usageApi.getKiroUsage(credentialUuid);
      setUsageInfo(info);
    } catch (e) {
      setUsageError(e instanceof Error ? e.message : String(e));
    } finally {
      setUsageLoading(false);
    }
  }, [credentialUuid, usageExpanded, usageInfo]);

  // 获取 Kiro 详细状态
  const handleCheckKiroStatus = useCallback(async () => {
    if (kiroStatusExpanded) {
      setKiroStatusExpanded(false);
      return;
    }

    setKiroStatusExpanded(true);
    setKiroStatusLoading(true);

    try {
      const status = await kiroCredentialApi.getCredentialStatus(credentialUuid);
      setKiroHealthScore(status.health_score || 0);
    } catch (e) {
      console.error("获取 Kiro 状态失败:", e);
    } finally {
      setKiroStatusLoading(false);
    }
  }, [credentialUuid, kiroStatusExpanded]);

  // 快速刷新 Kiro Token
  const handleQuickRefresh = useCallback(async () => {
    setKiroRefreshing(true);

    try {
      const result = await kiroCredentialApi.refreshCredential(credentialUuid);
      if (result.success) {
        console.log("Token 刷新成功:", result.message);
        if (onRefreshToken) {
          onRefreshToken();
        }
      } else {
        console.error("Token 刷新失败:", result.error || result.message);
      }
    } catch (e) {
      console.error("Token 刷新异常:", e);
    } finally {
      setKiroRefreshing(false);
    }
  }, [credentialUuid, onRefreshToken]);

  // 切换到本地
  const handleSwitchToLocal = useCallback(async () => {
    setSwitchingToLocal(true);
    setSwitchResult(null);

    try {
      const result = await switchKiroToLocal(credentialUuid);
      setSwitchResult(result);

      if (result.success) {
        console.log("切换到本地成功:", result.message);
        if (onSwitchToLocal) {
          onSwitchToLocal();
        }
      } else {
        console.error("切换到本地失败:", result.message);
      }

      // 5秒后自动清除结果提示
      setTimeout(() => {
        setSwitchResult(null);
      }, 5000);
    } catch (e) {
      console.error("切换到本地异常:", e);
      setSwitchResult({
        success: false,
        message: e instanceof Error ? e.message : String(e),
        requires_action: false,
        requires_kiro_restart: false,
      });
    } finally {
      setSwitchingToLocal(false);
    }
  }, [credentialUuid, onSwitchToLocal]);

  // 面板关闭方法
  const closeFingerprintPanel = useCallback(() => {
    setFingerprintExpanded(false);
  }, []);

  const closeUsagePanel = useCallback(() => {
    setUsageExpanded(false);
  }, []);

  const closeStatusPanel = useCallback(() => {
    setKiroStatusExpanded(false);
  }, []);

  return {
    // 指纹状态
    fingerprintInfo,
    fingerprintLoading,
    fingerprintExpanded,
    fingerprintCopied,
    // 用量状态
    usageInfo,
    usageLoading,
    usageExpanded,
    usageError,
    // Kiro 状态
    kiroHealthScore,
    kiroStatusLoading,
    kiroStatusExpanded,
    kiroRefreshing,
    // 切换到本地状态
    switchingToLocal,
    switchResult,
    // 操作方法
    handleCheckFingerprint,
    handleCopyMachineId,
    handleCheckUsage,
    handleCheckKiroStatus,
    handleQuickRefresh,
    handleSwitchToLocal,
    // 面板关闭方法
    closeFingerprintPanel,
    closeUsagePanel,
    closeStatusPanel,
  };
}
