/**
 * Format bytes into human-readable string
 */
export function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const size = bytes / Math.pow(1024, i);
  return `${size.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

/**
 * Format unix timestamp to human-readable date
 */
export function formatDate(timestamp: number | null): string {
  if (!timestamp) return 'Unknown';
  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

/**
 * Get a short path for display (last N segments)
 */
export function shortenPath(path: string, maxSegments = 4): string {
  const segments = path.split('/').filter(Boolean);
  if (segments.length <= maxSegments) return path;
  return '~/' + segments.slice(-maxSegments).join('/');
}
