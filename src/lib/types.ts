export interface FileEntry {
  name: string;
  path: string;
  size: number;
  is_dir: boolean;
  modified: number | null;
  extension: string | null;
  children: FileEntry[];
  /** Actual number of direct children (may exceed children.length if truncated) */
  child_count: number;
  /** True if children array was capped due to too many entries */
  truncated: boolean;
}
