import { api } from '@shared/api';
import { useQuery } from '@tanstack/react-query';
import { useCallback, useMemo, useState } from 'react';

/**
 * Query keys for preview data
 */
export const previewKeys = {
  all: ['preview'] as const,
  sd: (projectId: string, profileId: string) =>
    [...previewKeys.all, 'sd', projectId, profileId] as const,
  fsh: (projectId: string, profileId: string) =>
    [...previewKeys.all, 'fsh', projectId, profileId] as const,
};

/** Normalized preview data for components */
export interface PreviewData {
  content: string;
  filename: string;
}

/**
 * Hook for fetching SD JSON preview
 * Transforms API response to component-friendly format
 */
export function useSDJsonPreview(projectId: string, profileId: string) {
  return useQuery({
    queryKey: previewKeys.sd(projectId, profileId),
    queryFn: async (): Promise<PreviewData> => {
      console.log('[useSDJsonPreview] Fetching SD for:', { projectId, profileId });
      const response = await api.export.toSD(projectId, profileId);
      console.log('[useSDJsonPreview] Raw API response:', response);
      // Transform: content is JSON object, stringify it for display
      const content =
        typeof response.content === 'string'
          ? response.content
          : JSON.stringify(response.content, null, 2);
      const result = {
        content,
        filename: response.metadata?.filename ?? `${profileId}.json`,
      };
      console.log('[useSDJsonPreview] Transformed result:', result);
      return result;
    },
    enabled: !!projectId && !!profileId,
    staleTime: 30 * 1000, // 30 seconds - preview should be relatively fresh
  });
}

/**
 * Hook for fetching FSH preview
 * Transforms API response to component-friendly format
 */
export function useFSHPreview(projectId: string, profileId: string) {
  return useQuery({
    queryKey: previewKeys.fsh(projectId, profileId),
    queryFn: async (): Promise<PreviewData> => {
      const response = await api.export.toFSH(projectId, profileId);
      return {
        content: response.content,
        filename: response.metadata?.filename ?? `${profileId}.fsh`,
      };
    },
    enabled: !!projectId && !!profileId,
    staleTime: 30 * 1000,
  });
}

/**
 * Hook for managing preview panel state
 */
export function usePreviewPanel() {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');

  const toggleFullscreen = useCallback(() => {
    setIsFullscreen((prev) => !prev);
  }, []);

  const clearSearch = useCallback(() => {
    setSearchQuery('');
  }, []);

  return {
    isFullscreen,
    searchQuery,
    toggleFullscreen,
    setSearchQuery,
    clearSearch,
  };
}

/**
 * Hook for clipboard operations
 */
export function useClipboard() {
  const [copied, setCopied] = useState(false);

  const copy = useCallback(async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
      return true;
    } catch {
      return false;
    }
  }, []);

  return { copied, copy };
}

/**
 * Hook for file download
 */
export function useFileDownload() {
  const download = useCallback((content: string, filename: string, mimeType = 'text/plain') => {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  }, []);

  return { download };
}

/**
 * Compute diff between two strings
 * Returns arrays of added, removed, and unchanged lines
 */
export interface DiffLine {
  type: 'added' | 'removed' | 'unchanged' | 'modified';
  content: string;
  lineNumber: number;
  originalLineNumber?: number;
}

export function computeDiff(original: string, modified: string): DiffLine[] {
  const originalLines = original.split('\n');
  const modifiedLines = modified.split('\n');
  const result: DiffLine[] = [];

  // Simple line-by-line diff (LCS-based diff would be better for production)
  const maxLength = Math.max(originalLines.length, modifiedLines.length);

  for (let i = 0; i < maxLength; i++) {
    const origLine = originalLines[i];
    const modLine = modifiedLines[i];

    if (origLine === undefined && modLine !== undefined) {
      result.push({
        type: 'added',
        content: modLine,
        lineNumber: i + 1,
      });
    } else if (origLine !== undefined && modLine === undefined) {
      result.push({
        type: 'removed',
        content: origLine,
        lineNumber: i + 1,
        originalLineNumber: i + 1,
      });
    } else if (origLine === modLine) {
      result.push({
        type: 'unchanged',
        content: modLine,
        lineNumber: i + 1,
        originalLineNumber: i + 1,
      });
    } else {
      // Line modified
      result.push({
        type: 'removed',
        content: origLine,
        lineNumber: i + 1,
        originalLineNumber: i + 1,
      });
      result.push({
        type: 'added',
        content: modLine,
        lineNumber: i + 1,
      });
    }
  }

  return result;
}

/**
 * Hook for diff computation with memoization
 */
export function useDiff(original: string, modified: string) {
  return useMemo(() => computeDiff(original, modified), [original, modified]);
}
