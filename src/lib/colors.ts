// Extension-to-color map for file type coloring
// Categories: images=blue, code=green, documents=yellow, media=purple, folders=grey, other=orange

const FILE_COLORS: Record<string, string> = {
  // Code files — green shades
  js: '#4caf50',
  ts: '#388e3c',
  jsx: '#66bb6a',
  tsx: '#43a047',
  py: '#2e7d32',
  rs: '#1b5e20',
  go: '#00c853',
  rb: '#69f0ae',
  java: '#a5d6a7',
  c: '#81c784',
  cpp: '#66bb6a',
  h: '#a5d6a7',
  cs: '#4caf50',
  php: '#388e3c',
  swift: '#2e7d32',
  kt: '#1b5e20',
  vue: '#00e676',
  svelte: '#ff3e00',
  html: '#ef6c00',
  css: '#42a5f5',
  scss: '#1e88e5',
  less: '#1565c0',
  json: '#78909c',
  xml: '#90a4ae',
  yaml: '#78909c',
  yml: '#78909c',
  toml: '#78909c',
  sh: '#a5d6a7',
  bash: '#a5d6a7',
  sql: '#4db6ac',

  // Images — blue shades
  png: '#2196f3',
  jpg: '#1976d2',
  jpeg: '#1565c0',
  gif: '#42a5f5',
  svg: '#64b5f6',
  webp: '#0d47a1',
  ico: '#1e88e5',
  bmp: '#1976d2',
  tiff: '#1565c0',

  // Documents — yellow/amber shades
  pdf: '#ffa000',
  doc: '#ffb300',
  docx: '#ffb300',
  xls: '#43a047',
  xlsx: '#43a047',
  ppt: '#e65100',
  pptx: '#e65100',
  txt: '#ffd54f',
  md: '#ffe082',
  rtf: '#ffca28',
  csv: '#c0ca33',

  // Media — purple shades
  mp3: '#9c27b0',
  mp4: '#7b1fa2',
  avi: '#6a1b9a',
  mov: '#8e24aa',
  wav: '#ab47bc',
  flac: '#ba68c8',
  mkv: '#7b1fa2',
  webm: '#6a1b9a',
  ogg: '#ce93d8',

  // Archives — brown/red shades
  zip: '#795548',
  tar: '#6d4c41',
  gz: '#5d4037',
  rar: '#4e342e',
  '7z': '#3e2723',

  // Config/lock files
  lock: '#607d8b',
  env: '#ff5722',
  gitignore: '#607d8b',
};

const FOLDER_COLOR = '#78909c';
const DEFAULT_FILE_COLOR = '#ff9800';

export function getFileColor(extension: string | null, isDir: boolean): string {
  if (isDir) return FOLDER_COLOR;
  if (!extension) return DEFAULT_FILE_COLOR;
  return FILE_COLORS[extension] || DEFAULT_FILE_COLOR;
}

export function getFileColorBright(extension: string | null, isDir: boolean): string {
  const base = getFileColor(extension, isDir);
  // Lighten the color for hover by mixing with white
  return lightenColor(base, 0.2);
}

function lightenColor(hex: string, amount: number): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  const nr = Math.min(255, Math.round(r + (255 - r) * amount));
  const ng = Math.min(255, Math.round(g + (255 - g) * amount));
  const nb = Math.min(255, Math.round(b + (255 - b) * amount));
  return `#${nr.toString(16).padStart(2, '0')}${ng.toString(16).padStart(2, '0')}${nb.toString(16).padStart(2, '0')}`;
}

/**
 * Get the file type category for display
 */
export function getFileTypeCategory(extension: string | null, isDir: boolean): string {
  if (isDir) return 'Folder';
  if (!extension) return 'Other';

  const codeExts = ['js','ts','jsx','tsx','py','rs','go','rb','java','c','cpp','h','cs','php','swift','kt','vue','svelte','html','css','scss','less','json','xml','yaml','yml','toml','sh','bash','sql'];
  const imageExts = ['png','jpg','jpeg','gif','svg','webp','ico','bmp','tiff'];
  const docExts = ['pdf','doc','docx','xls','xlsx','ppt','pptx','txt','md','rtf','csv'];
  const mediaExts = ['mp3','mp4','avi','mov','wav','flac','mkv','webm','ogg'];
  const archiveExts = ['zip','tar','gz','rar','7z'];

  if (codeExts.includes(extension)) return 'Code';
  if (imageExts.includes(extension)) return 'Image';
  if (docExts.includes(extension)) return 'Document';
  if (mediaExts.includes(extension)) return 'Media';
  if (archiveExts.includes(extension)) return 'Archive';
  return 'Other';
}

export { FOLDER_COLOR, DEFAULT_FILE_COLOR };
