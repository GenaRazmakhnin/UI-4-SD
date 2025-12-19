import { packageKeys } from '@entities/package/api/queries';
import { api } from '@shared/api';
import type {
  InstallJob,
  Package,
  PackageInstallProgress,
  PackageInstallStatus,
} from '@shared/types';
import { useQueryClient } from '@tanstack/react-query';
import { useCallback, useRef, useState } from 'react';

export interface UsePackageInstallPollingOptions {
  onComplete?: (pkg: Package) => void;
  onError?: (error: string) => void;
  onProgress?: (progress: PackageInstallProgress) => void;
  /** Polling interval in ms (default: 500) */
  pollInterval?: number;
}

export interface UsePackageInstallPollingReturn {
  status: PackageInstallStatus;
  progress: number;
  message: string;
  currentPackageId: string | null;
  downloadedBytes: number;
  totalBytes: number | null;
  install: (packageId: string) => Promise<void>;
  cancel: () => void;
  reset: () => void;
}

/**
 * Hook for installing packages with polling-based progress updates.
 * More reliable than SSE in various environments (proxies, etc).
 */
export function usePackageInstallPolling(
  options?: UsePackageInstallPollingOptions
): UsePackageInstallPollingReturn {
  const queryClient = useQueryClient();
  const pollIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const cancelledRef = useRef(false);

  const [status, setStatus] = useState<PackageInstallStatus>('idle');
  const [progress, setProgress] = useState(0);
  const [message, setMessage] = useState('');
  const [currentPackageId, setCurrentPackageId] = useState<string | null>(null);
  const [downloadedBytes, setDownloadedBytes] = useState(0);
  const [totalBytes, setTotalBytes] = useState<number | null>(null);

  const pollInterval = options?.pollInterval ?? 500;

  const stopPolling = useCallback(() => {
    if (pollIntervalRef.current) {
      clearInterval(pollIntervalRef.current);
      pollIntervalRef.current = null;
    }
  }, []);

  const updateFromJob = useCallback(
    (job: InstallJob) => {
      // Map job status to our status
      const statusMap: Record<string, PackageInstallStatus> = {
        pending: 'installing',
        downloading: 'installing',
        extracting: 'extracting',
        indexing: 'indexing',
        completed: 'installed',
        failed: 'error',
      };

      const newStatus = statusMap[job.status] || 'installing';
      setStatus(newStatus);
      setProgress(job.progress);
      setMessage(job.message || '');
      setDownloadedBytes(job.downloadedBytes || 0);
      setTotalBytes(job.totalBytes || null);

      options?.onProgress?.({
        packageId: job.packageId,
        status: newStatus,
        progress: job.progress,
        message: job.message,
        downloadedBytes: job.downloadedBytes,
        totalBytes: job.totalBytes,
      });

      return { newStatus, job };
    },
    [options]
  );

  const install = useCallback(
    async (packageId: string) => {
      // Stop any existing polling
      stopPolling();
      cancelledRef.current = false;

      // Reset state
      setCurrentPackageId(packageId);
      setStatus('installing');
      setProgress(0);
      setMessage('Starting installation...');
      setDownloadedBytes(0);
      setTotalBytes(null);

      try {
        // Start the install job
        const job = await api.packages.startInstall(packageId);

        if (cancelledRef.current) return;

        // Start polling for progress
        const pollForProgress = async () => {
          if (cancelledRef.current) {
            stopPolling();
            return;
          }

          try {
            const currentJob = await api.packages.getInstallJob(job.jobId);
            const { newStatus, job: updatedJob } = updateFromJob(currentJob);

            // Check if installation is complete
            if (newStatus === 'installed') {
              stopPolling();
              queryClient.invalidateQueries({ queryKey: packageKeys.all });
              if (updatedJob.package) {
                options?.onComplete?.(updatedJob.package);
              }
            } else if (newStatus === 'error') {
              stopPolling();
              options?.onError?.(updatedJob.error || 'Installation failed');
            }
          } catch (error) {
            stopPolling();
            setStatus('error');
            setMessage(error instanceof Error ? error.message : 'Failed to check status');
            options?.onError?.(error instanceof Error ? error.message : 'Failed to check status');
          }
        };

        // Initial update
        updateFromJob(job);

        // Start polling
        pollIntervalRef.current = setInterval(pollForProgress, pollInterval);
      } catch (error) {
        setStatus('error');
        setMessage(error instanceof Error ? error.message : 'Failed to start installation');
        options?.onError?.(error instanceof Error ? error.message : 'Failed to start installation');
      }
    },
    [queryClient, options, pollInterval, stopPolling, updateFromJob]
  );

  const cancel = useCallback(() => {
    cancelledRef.current = true;
    stopPolling();
    setStatus('idle');
    setMessage('Installation cancelled');
  }, [stopPolling]);

  const reset = useCallback(() => {
    cancel();
    setCurrentPackageId(null);
    setProgress(0);
    setDownloadedBytes(0);
    setTotalBytes(null);
    setMessage('');
  }, [cancel]);

  return {
    status,
    progress,
    message,
    currentPackageId,
    downloadedBytes,
    totalBytes,
    install,
    cancel,
    reset,
  };
}
