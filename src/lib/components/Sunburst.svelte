<script lang="ts">
  import { onMount } from 'svelte';
  import * as d3 from 'd3';
  import type { FileEntry } from '$lib/types';
  import { getFileColor } from '$lib/colors';
  import { formatSize } from '$lib/utils';

  interface Props {
    data: FileEntry;
    onNavigate: (path: string) => void;
    onHover: (entry: FileEntry | null, event?: MouseEvent) => void;
    onFileOpen: (entry: FileEntry) => void;
    onRevealInFinder: (entry: FileEntry) => void;
  }

  let { data, onNavigate, onHover, onFileOpen, onRevealInFinder }: Props = $props();

  let svgElement: SVGSVGElement;
  let width = $state(600);
  let height = $state(600);

  const TRANSITION_MS = 350;
  const MAX_CHILDREN_SHOWN = 15;
  const FLAT_FOLDER_THRESHOLD = 20;
  const MAX_GRANDCHILD_SEGMENTS = 200;
  const MIN_ARC_ANGLE = 0.01; // radians — segments smaller than this are invisible

  // Convert FileEntry to d3 hierarchy format with flat-folder grouping
  function toHierarchy(entry: FileEntry): any {
    let children = entry.children;

    // Flat-folder mitigation: if >20 children, group smallest into "other"
    if (children.length > FLAT_FOLDER_THRESHOLD) {
      const sorted = [...children].sort((a, b) => b.size - a.size);
      const top = sorted.slice(0, MAX_CHILDREN_SHOWN);
      const rest = sorted.slice(MAX_CHILDREN_SHOWN);
      const otherSize = rest.reduce((sum, c) => sum + c.size, 0);
      top.push({
        name: `${rest.length} other items`,
        path: entry.path + '/__other__',
        size: otherSize,
        is_dir: false,
        modified: null,
        extension: '__other__',
        children: [],
        child_count: 0,
        truncated: false,
      });
      children = top;
    }

    return {
      name: entry.name,
      path: entry.path,
      is_dir: entry.is_dir,
      extension: entry.extension,
      modified: entry.modified,
      size: entry.size,
      child_count: entry.child_count,
      truncated: entry.truncated,
      originalEntry: entry,
      children: children.length > 0
        ? children.map(toHierarchy)
        : undefined,
      value: children.length === 0 ? Math.max(entry.size, 1) : undefined,
    };
  }

  /**
   * Estimate how many characters fit across a given pixel width,
   * assuming an average character width factor for the given font size.
   */
  function charsForWidth(pixelWidth: number, fontSize: number): number {
    // Average char width is roughly 0.6 * fontSize for typical sans-serif fonts
    const avgCharWidth = fontSize * 0.6;
    return Math.max(3, Math.floor(pixelWidth / avgCharWidth));
  }

  function render() {
    if (!svgElement || !data) return;

    const svg = d3.select(svgElement);
    svg.selectAll('*').remove();

    const size = Math.min(width, height);
    const radius = size / 2 * 0.95;
    const innerRadius = radius * 0.18;
    const ringWidth = (radius - innerRadius) / 2;

    const g = svg
      .attr('viewBox', `${-width / 2} ${-height / 2} ${width} ${height}`)
      .append('g');

    // Build hierarchy
    const hierData = toHierarchy(data);
    const root = d3
      .hierarchy(hierData)
      .sum((d: any) => d.value || 0)
      .sort((a: any, b: any) => (b.value || 0) - (a.value || 0));

    // Partition layout
    const partition = d3.partition<any>().size([2 * Math.PI, radius]);
    partition(root);

    // Arc generator
    const arc = d3
      .arc<d3.HierarchyRectangularNode<any>>()
      .startAngle((d) => d.x0)
      .endAngle((d) => d.x1)
      .padAngle((d) => Math.min((d.x1 - d.x0) / 2, 0.005))
      .padRadius(radius / 2)
      .innerRadius((d) => {
        if (d.depth === 0) return 0;
        if (d.depth === 1) return innerRadius;
        return innerRadius + (d.depth - 1) * ringWidth;
      })
      .outerRadius((d) => {
        if (d.depth === 0) return innerRadius;
        return innerRadius + d.depth * ringWidth;
      });

    // --- Center circle (current directory) ---
    const centerGroup = g.append('g').attr('class', 'center');
    const isEmpty = data.children.length === 0;

    centerGroup.append('circle')
      .attr('r', innerRadius)
      .attr('fill', '#2d3748')
      .attr('stroke', 'rgba(255,255,255,0.15)')
      .attr('stroke-width', 2)
      .attr('cursor', 'pointer')
      .on('click', () => {
        const parentPath = data.path.split('/').slice(0, -1).join('/') || '/';
        onNavigate(parentPath);
      })
      .on('mouseenter', (event: MouseEvent) => {
        onHover(data, event);
        d3.select(event.currentTarget as Element)
          .transition().duration(150)
          .attr('fill', '#4a5568');
      })
      .on('mouseleave', (event: MouseEvent) => {
        onHover(null);
        d3.select(event.currentTarget as Element)
          .transition().duration(150)
          .attr('fill', '#2d3748');
      });

    // Improved center text: dynamically compute how many chars fit
    // using the usable width (diameter minus some padding) and font size.
    const nameFontSize = Math.max(10, innerRadius * 0.22);
    const usableDiameter = innerRadius * 2 * 0.8; // 80% of diameter for padding
    const maxCenterChars = charsForWidth(usableDiameter, nameFontSize);

    if (isEmpty) {
      // --- Empty directory: show "Empty folder" message ---
      centerGroup.append('text')
        .attr('text-anchor', 'middle')
        .attr('dy', '-0.8em')
        .attr('fill', '#e2e8f0')
        .attr('font-size', `${nameFontSize}px`)
        .attr('font-weight', '600')
        .attr('pointer-events', 'none')
        .text(data.name.length > maxCenterChars ? data.name.slice(0, maxCenterChars - 1) + '\u2026' : data.name);

      centerGroup.append('text')
        .attr('text-anchor', 'middle')
        .attr('dy', '0.7em')
        .attr('fill', '#a0aec0')
        .attr('font-size', `${Math.max(9, innerRadius * 0.17)}px`)
        .attr('font-style', 'italic')
        .attr('pointer-events', 'none')
        .text('Empty folder');

      // Up hint
      centerGroup.append('text')
        .attr('text-anchor', 'middle')
        .attr('dy', '2.5em')
        .attr('fill', '#718096')
        .attr('font-size', `${Math.max(7, innerRadius * 0.12)}px`)
        .attr('pointer-events', 'none')
        .text('\u25B2 parent');
    } else {
      // --- Normal center text ---
      // Center directory name
      centerGroup.append('text')
        .attr('text-anchor', 'middle')
        .attr('dy', '-0.5em')
        .attr('fill', '#e2e8f0')
        .attr('font-size', `${nameFontSize}px`)
        .attr('font-weight', '600')
        .attr('pointer-events', 'none')
        .text(data.name.length > maxCenterChars ? data.name.slice(0, maxCenterChars - 1) + '\u2026' : data.name);

      // Center size
      centerGroup.append('text')
        .attr('text-anchor', 'middle')
        .attr('dy', '1em')
        .attr('fill', '#a0aec0')
        .attr('font-size', `${Math.max(8, innerRadius * 0.16)}px`)
        .attr('pointer-events', 'none')
        .text(formatSize(data.size));

      // Truncation indicator: "(showing N of M items)"
      const visibleChildren = data.children.length;
      const totalChildren = data.child_count ?? visibleChildren;
      const isTruncated = data.truncated || totalChildren > visibleChildren;

      if (isTruncated) {
        centerGroup.append('text')
          .attr('text-anchor', 'middle')
          .attr('dy', '2.2em')
          .attr('fill', '#a0aec0')
          .attr('font-size', `${Math.max(7, innerRadius * 0.12)}px`)
          .attr('pointer-events', 'none')
          .text(`(showing ${visibleChildren} of ${totalChildren} items)`);

        // Up hint pushed down to make room
        centerGroup.append('text')
          .attr('text-anchor', 'middle')
          .attr('dy', '3.6em')
          .attr('fill', '#718096')
          .attr('font-size', `${Math.max(7, innerRadius * 0.12)}px`)
          .attr('pointer-events', 'none')
          .text('\u25B2 parent');
      } else {
        // Center "up" hint (normal position)
        centerGroup.append('text')
          .attr('text-anchor', 'middle')
          .attr('dy', '2.5em')
          .attr('fill', '#718096')
          .attr('font-size', `${Math.max(7, innerRadius * 0.12)}px`)
          .attr('pointer-events', 'none')
          .text('\u25B2 parent');
      }
    }

    // --- Ring segments ---
    // Get all descendants at depth > 0
    let allNodes = root.descendants().filter((d: any) => d.depth > 0);

    // Minimum arc angle threshold: skip segments that are too small to see
    allNodes = allNodes.filter((d: any) => (d.x1 - d.x0) >= MIN_ARC_ANGLE);

    // Lazy rendering for grandchild ring (depth 2):
    // If there are more than MAX_GRANDCHILD_SEGMENTS at depth 2,
    // keep only the largest ones and add a visual "..." indicator.
    const depth1Nodes = allNodes.filter((d: any) => d.depth === 1);
    let depth2Nodes = allNodes.filter((d: any) => d.depth === 2);
    let grandchildTruncated = false;
    let grandchildOmittedCount = 0;

    if (depth2Nodes.length > MAX_GRANDCHILD_SEGMENTS) {
      grandchildTruncated = true;
      grandchildOmittedCount = depth2Nodes.length - MAX_GRANDCHILD_SEGMENTS;
      // Sort by arc size descending, keep the largest
      depth2Nodes.sort((a: any, b: any) => (b.x1 - b.x0) - (a.x1 - a.x0));
      depth2Nodes = depth2Nodes.slice(0, MAX_GRANDCHILD_SEGMENTS);
    }

    const nodes = [...depth1Nodes, ...depth2Nodes];

    const paths = g
      .selectAll('path.segment')
      .data(nodes)
      .join('path')
      .attr('class', 'segment')
      .attr('d', arc as any)
      .attr('fill', (d: any) => {
        if (d.data.extension === '__other__') return '#4a5568';
        return getFileColor(d.data.extension, d.data.is_dir);
      })
      .attr('fill-opacity', (d: any) => (d.depth === 1 ? 0.85 : 0.6))
      .attr('stroke', '#1a202c')
      .attr('stroke-width', 0.5)
      .attr('cursor', (d: any) => {
        if (d.data.extension === '__other__') return 'default';
        return 'pointer';
      })
      .on('click', (event: MouseEvent, d: any) => {
        if (d.data.extension === '__other__') return;
        if (event.metaKey) {
          const entry = d.data.originalEntry || d.data;
          onRevealInFinder(entry);
          return;
        }
        if (d.data.is_dir) {
          onNavigate(d.data.path);
        }
      })
      .on('dblclick', (_event: MouseEvent, d: any) => {
        if (d.data.extension === '__other__') return;
        if (!d.data.is_dir) {
          const entry = d.data.originalEntry || d.data;
          onFileOpen(entry);
        }
      })
      .on('mouseenter', (event: MouseEvent, d: any) => {
        const entry = d.data.originalEntry || d.data;
        onHover(entry, event);
        d3.select(event.currentTarget as Element)
          .transition().duration(100)
          .attr('fill-opacity', 1)
          .attr('stroke', '#e2e8f0')
          .attr('stroke-width', 2);
      })
      .on('mouseleave', (event: MouseEvent, d: any) => {
        onHover(null);
        d3.select(event.currentTarget as Element)
          .transition().duration(150)
          .attr('fill-opacity', d.depth === 1 ? 0.85 : 0.6)
          .attr('stroke', '#1a202c')
          .attr('stroke-width', 0.5);
      });

    // Entrance animation
    paths
      .attr('opacity', 0)
      .transition()
      .duration(TRANSITION_MS)
      .ease(d3.easeCubicOut)
      .attr('opacity', 1);

    // --- Grandchild truncation "..." indicator ---
    if (grandchildTruncated) {
      const outerRingOuter = innerRadius + 2 * ringWidth;
      const outerRingInner = innerRadius + ringWidth;
      const indicatorR = (outerRingOuter + outerRingInner) / 2;

      g.append('text')
        .attr('class', 'grandchild-overflow')
        .attr('x', 0)
        .attr('y', -indicatorR)
        .attr('text-anchor', 'middle')
        .attr('dy', '0.35em')
        .attr('fill', '#a0aec0')
        .attr('font-size', `${Math.max(8, ringWidth * 0.1)}px`)
        .attr('pointer-events', 'none')
        .text(`+${grandchildOmittedCount} more`)
        .attr('opacity', 0)
        .transition()
        .delay(TRANSITION_MS * 0.6)
        .duration(TRANSITION_MS * 0.4)
        .attr('opacity', 0.7);
    }

    // --- Labels for large segments (depth 1 only) ---
    const labelNodes = root.descendants().filter((d: any) => {
      if (d.depth !== 1) return false;
      const angle = d.x1 - d.x0;
      return angle > 0.25;
    });

    g.selectAll('text.label')
      .data(labelNodes)
      .join('text')
      .attr('class', 'label')
      .attr('transform', (d: any) => {
        const midAngle = (d.x0 + d.x1) / 2;
        const angleDeg = midAngle * (180 / Math.PI) - 90;
        const r = innerRadius + ringWidth / 2;
        const flip = angleDeg > 90 && angleDeg < 270;
        return `rotate(${angleDeg}) translate(${r},0) rotate(${flip ? 180 : 0})`;
      })
      .attr('text-anchor', 'middle')
      .attr('dy', '0.35em')
      .attr('fill', '#fff')
      .attr('font-size', `${Math.max(8, ringWidth * 0.08)}px`)
      .attr('font-weight', '500')
      .attr('pointer-events', 'none')
      .text((d: any) => {
        const name = d.data.name;
        const arcLen = (d.x1 - d.x0) * (innerRadius + ringWidth / 2);
        const maxChars = Math.max(3, Math.floor(arcLen / 7));
        return name.length > maxChars ? name.slice(0, maxChars - 1) + '\u2026' : name;
      })
      .attr('opacity', 0)
      .transition()
      .delay(TRANSITION_MS * 0.5)
      .duration(TRANSITION_MS * 0.5)
      .attr('opacity', 0.9);

    // --- Small icon indicators for medium segments ---
    const iconNodes = root.descendants().filter((d: any) => {
      if (d.depth !== 1) return false;
      const angle = d.x1 - d.x0;
      return angle > 0.12 && angle <= 0.25;
    });

    g.selectAll('text.icon')
      .data(iconNodes)
      .join('text')
      .attr('class', 'icon')
      .attr('transform', (d: any) => {
        const midAngle = (d.x0 + d.x1) / 2;
        const angleDeg = midAngle * (180 / Math.PI) - 90;
        const r = innerRadius + ringWidth / 2;
        return `rotate(${angleDeg}) translate(${r},0) rotate(${-angleDeg + 90})`;
      })
      .attr('text-anchor', 'middle')
      .attr('dy', '0.35em')
      .attr('fill', '#fff')
      .attr('font-size', '12px')
      .attr('pointer-events', 'none')
      .text((d: any) => d.data.is_dir ? '\uD83D\uDCC1' : '\uD83D\uDCC4')
      .attr('opacity', 0)
      .transition()
      .delay(TRANSITION_MS * 0.5)
      .duration(TRANSITION_MS * 0.5)
      .attr('opacity', 0.7);
  }

  // Observe container size for responsiveness (debounced)
  let containerEl: HTMLDivElement;
  let resizeTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    const observer = new ResizeObserver((entries) => {
      // Debounce: wait 100ms of inactivity before applying new dimensions
      if (resizeTimer) clearTimeout(resizeTimer);
      resizeTimer = setTimeout(() => {
        for (const entry of entries) {
          const { width: w, height: h } = entry.contentRect;
          width = w;
          height = h;
        }
      }, 100);
    });
    if (containerEl) {
      observer.observe(containerEl);
      // Initial size — set immediately without debounce
      const rect = containerEl.getBoundingClientRect();
      width = rect.width;
      height = rect.height;
    }
    return () => {
      if (resizeTimer) clearTimeout(resizeTimer);
      observer.disconnect();
    };
  });

  // Re-render when data or dimensions change
  $effect(() => {
    data;
    width;
    height;
    render();
  });
</script>

<div class="sunburst-container" bind:this={containerEl}>
  <svg bind:this={svgElement}></svg>
</div>

<style>
  .sunburst-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  svg {
    width: 100%;
    height: 100%;
  }
</style>
