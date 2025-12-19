import { packageKeys } from '@entities/package/api/queries';
import { api } from '@shared/api';
import type {
  InstallEvent,
  Package,
  PackageInstallProgress,
  PackageInstallStatus,
} from '@shared/types';
import { useQueryClient } from '@tanstack/react-query';
import { useCallback, useRef, useState } from 'react';

export interface UsePackageInstallSSEOptions {
  onComplete?: (pkg: Package) => void;
  onError?: (error: string, code: string) => void;
  onProgress?: (progress: PackageInstallProgress) => void;
}

export interface UsePackageInstallSSEReturn {
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
 * Hook for installing packages with SSE-based progress updates.
 * Uses fetch with ReadableStream to handle POST SSE responses.
 */
export function usePackageInstallSSE(
  options?: UsePackageInstallSSEOptions
): UsePackageInstallSSEReturn {
  const queryClient = useQueryClient();
  const abortControllerRef = useRef<AbortController | null>(null);

  const [status, setStatus] = useState<PackageInstallStatus>('idle');
  const [progress, setProgress] = useState(0);
  const [message, setMessage] = useState('');
  const [currentPackageId, setCurrentPackageId] = useState<string | null>(null);
  const [downloadedBytes, setDownloadedBytes] = useState(0);
  const [totalBytes, setTotalBytes] = useState<number | null>(null);

  const updateProgress = useCallback(
    (update: Partial<PackageInstallProgress>) => {
      const currentProgress: PackageInstallProgress = {
        packageId: currentPackageId || '',
        status,
        progress,
        message,
        downloadedBytes,
        totalBytes: totalBytes ?? undefined,
        ...update,
      };
      options?.onProgress?.(currentProgress);
    },
    [currentPackageId, status, progress, message, downloadedBytes, totalBytes, options]
  );

  const handleEvent = useCallback(
    (event: InstallEvent) => {
      switch (event.type) {
        case 'start':
          setStatus('installing');
          setProgress(0);
          setMessage('Starting installation...');
          if (event.total_bytes) {
            setTotalBytes(event.total_bytes);
          }
          updateProgress({
            status: 'installing',
            progress: 0,
            message: 'Starting installation...',
            totalBytes: event.total_bytes,
          });
          break;

        case 'progress': {
          setProgress(event.percentage);
          setDownloadedBytes(event.downloaded_bytes);
          if (event.total_bytes) {
            setTotalBytes(event.total_bytes);
          }
          const downloadMsg = event.total_bytes
            ? `Downloading... ${formatBytes(event.downloaded_bytes)} / ${formatBytes(event.total_bytes)}`
            : `Downloading... ${formatBytes(event.downloaded_bytes)}`;
          setMessage(downloadMsg);
          updateProgress({
            status: 'installing',
            progress: event.percentage,
            message: downloadMsg,
            downloadedBytes: event.downloaded_bytes,
            totalBytes: event.total_bytes,
          });
          break;
        }

        case 'extracting':
          setStatus('extracting');
          setProgress(100);
          setMessage('Extracting package...');
          updateProgress({
            status: 'extracting',
            progress: 100,
            message: 'Extracting package...',
          });
          break;

        case 'indexing':
          setStatus('indexing');
          setMessage('Indexing resources...');
          updateProgress({
            status: 'indexing',
            message: 'Indexing resources...',
          });
          break;

        case 'complete':
          setStatus('installed');
          setProgress(100);
          setMessage('Installation complete!');
          // Invalidate package queries to refresh the list
          queryClient.invalidateQueries({ queryKey: packageKeys.all });
          options?.onComplete?.(event.package);
          break;

        case 'error':
          setStatus('error');
          setMessage(event.message);
          options?.onError?.(event.message, event.code);
          break;
      }
    },
    [queryClient, options, updateProgress]
  );

  const install = useCallback(
    async (packageId: string) => {
      // Cancel any existing installation
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }

      // Create new abort controller
      const controller = new AbortController();
      abortControllerRef.current = controller;

      // Reset state
      setCurrentPackageId(packageId);
      setStatus('installing');
      setProgress(0);
      setMessage('Connecting...');
      setDownloadedBytes(0);
      setTotalBytes(null);

      try {
        await api.packages.installWithProgress(packageId, handleEvent, {
          signal: controller.signal,
        });
      } catch (error) {
        if (error instanceof Error && error.name === 'AbortError') {
          // Installation was cancelled
          setStatus('idle');
          setMessage('Installation cancelled');
        } else {
          setStatus('error');
          setMessage(error instanceof Error ? error.message : 'Installation failed');
          options?.onError?.(
            error instanceof Error ? error.message : 'Installation failed',
            'UNKNOWN_ERROR'
          );
        }
      } finally {
        if (abortControllerRef.current === controller) {
          abortControllerRef.current = null;
        }
      }
    },
    [handleEvent, options]
  );

  const cancel = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
    setStatus('idle');
    setMessage('');
  }, []);

  const reset = useCallback(() => {
    cancel();
    setCurrentPackageId(null);
    setProgress(0);
    setDownloadedBytes(0);
    setTotalBytes(null);
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

/**
 * Format bytes to human-readable string
 */
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / k ** i).toFixed(1))} ${sizes[i]}`;
}
