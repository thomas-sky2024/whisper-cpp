<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  // Chấp nhận binding hai chiều (Svelte 5 - $bindable() if used, but here standard prop for compatibility)
  export let activeModelFilename = "ggml-large-v3-turbo-q8_0.bin";

  interface ModelItem {
    id: string;
    name: string;
    description: string;
    size_str: string;
    filename: string;
    is_downloaded: boolean;
  }

  interface DownloadProgress {
    model_id: string;
    downloaded_mb: number;
    total_mb: number;
    percent: number;
    speed_mbps: number;
    done: boolean;
    error: string | null;
  }

  let models: ModelItem[] = [];
  let downloadProgress: Record<string, DownloadProgress> = {};
  let downloadErrors: Record<string, string> = {};
  let unlistenProgress: (() => void) | null = null;

  async function fetchModels() {
    try {
      models = await invoke('get_models_status');
    } catch (e) {
      console.error("Lỗi khi tải trạng thái model:", e);
    }
  }

  async function handleDownload(id: string) {
    if (isDownloading(id, downloadProgress)) return;
    delete downloadErrors[id];
    downloadErrors = { ...downloadErrors };
    downloadProgress = {
      ...downloadProgress,
      [id]: { model_id: id, downloaded_mb: 0, total_mb: 0, percent: 0, speed_mbps: 0, done: false, error: null }
    };

    try {
      await invoke('download_model_by_id', { modelId: id });
    } catch (e: any) {
      downloadErrors[id] = String(e);
      downloadErrors = { ...downloadErrors };
      delete downloadProgress[id];
      downloadProgress = { ...downloadProgress };
    }
  }

  function selectModel(filename: string) {
    activeModelFilename = filename;
  }

  function isDownloading(id: string, progress: Record<string, DownloadProgress>): boolean {
    const p = progress[id];
    return !!p && !p.done;
  }

  onMount(async () => {
    await fetchModels();

    unlistenProgress = await listen<DownloadProgress>('model-download-progress', async (event) => {
      const p = event.payload;
      downloadProgress = { ...downloadProgress, [p.model_id]: p };

      if (p.done) {
        if (p.error) {
          downloadErrors[p.model_id] = p.error;
          downloadErrors = { ...downloadErrors };
        } else {
          // [AUTO-APPLY LOGIC] Tìm model tương ứng và tự động chọn
          const loadedModel = models.find(m => m.id === p.model_id);
          if (loadedModel) {
            activeModelFilename = loadedModel.filename;
          }
        }
        
        await fetchModels();
        
        setTimeout(() => {
          delete downloadProgress[p.model_id];
          downloadProgress = { ...downloadProgress };
        }, 500);
      }
    });
  });

  onDestroy(() => {
    unlistenProgress?.();
  });
</script>

<div class="model-selector">
  <div class="header">
    <div class="header-top">
      <div class="header-info">
        <h3 class="title">Cấu hình Whisper Engine</h3>
        <p class="subtitle">Chọn model phù hợp để bắt đầu chuyển văn bản</p>
      </div>
      <button class="btn btn-secondary btn-small" on:click={() => invoke('open_models_dir')}>
        <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/></svg>
        Mở thư mục
      </button>
    </div>
  </div>

  <div class="model-grid">
    {#each models as model}
      {@const prog = downloadProgress[model.id]}
      {@const downloading = isDownloading(model.id, downloadProgress)}
      {@const error = downloadErrors[model.id]}
      {@const isSelected = activeModelFilename === model.filename}

      <div class="card {isSelected ? 'selected' : ''} {!model.is_downloaded && !downloading ? 'dimmed' : ''}">
        <div class="card-content">
          <div class="info-group">
            <div class="name-row">
              <span class="model-name">{model.name}</span>
              {#if isSelected && model.is_downloaded}
                <span class="badge active">Đang dùng</span>
              {:else if model.is_downloaded}
                <span class="badge downloaded">Sẵn sàng</span>
              {:else if downloading}
                <span class="badge downloading">Đang tải...</span>
              {:else}
                <span class="badge missing">Chưa có</span>
              {/if}
            </div>
            <p class="description">{model.description}</p>
            <div class="meta">
              <span class="size-tag">{model.size_str}</span>
            </div>
          </div>

          <div class="action-group">
            {#if downloading && prog}
              <div class="progress-ring">
                <svg viewBox="0 0 36 36" class="circular-chart">
                  <path class="circle-bg" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
                  <path class="circle" stroke-dasharray="{prog.percent}, 100" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
                  <text x="18" y="20.35" class="percentage">{Math.round(prog.percent)}%</text>
                </svg>
              </div>
            {:else if model.is_downloaded}
              {#if isSelected}
                <div class="check-icon">
                  <svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"/></svg>
                </div>
              {:else}
                <button class="btn btn-primary" on:click={() => selectModel(model.filename)}>
                  Áp dụng
                </button>
              {/if}
            {:else}
              <button 
                class="btn btn-download" 
                on:click={() => handleDownload(model.id)}
                disabled={downloading}
              >
                <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/></svg>
                Tải ngay
              </button>
            {/if}
          </div>
        </div>

        {#if downloading && prog}
          <div class="progress-footer">
            <div class="progress-bar-container">
              <div class="progress-bar-fill" style="width: {prog.percent}%"></div>
            </div>
            <div class="progress-stats">
              <span>{prog.downloaded_mb.toFixed(0)} MB / {prog.total_mb.toFixed(0)} MB</span>
              <span class="speed">{prog.speed_mbps.toFixed(1)} MB/s</span>
            </div>
          </div>
        {/if}

        {#if error}
          <div class="error-panel">
             <span class="error-text">⚠️ Tải lỗi: {error}</span>
             <button class="retry-link" on:click={() => handleDownload(model.id)}>Thử lại</button>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .model-selector {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    padding: 0.5rem 0;
  }

  .header {
    border-bottom: 1px solid #f3f4f6;
    padding-bottom: 0.75rem;
  }

  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .header-info {
    flex: 1;
  }

  .title {
    font-size: 0.95rem;
    font-weight: 700;
    color: #111827;
    margin: 0;
  }

  .audit-item-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .subtitle {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.15rem 0 0 0;
  }

  .model-grid {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .card {
    background: #ffffff;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1rem;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }

  .card:hover {
    border-color: #d1d5db;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
  }

  .card.selected {
    border-color: #3b82f6;
    background: #f0f7ff;
    box-shadow: 0 0 0 1px #3b82f6;
  }

  .card.dimmed {
    background: #fafafa;
  }

  .card-content {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
  }

  .info-group {
    flex: 1;
    min-width: 0;
  }

  .name-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.35rem;
    flex-wrap: wrap;
  }

  .model-name {
    font-size: 0.9rem;
    font-weight: 600;
    color: #1f2937;
  }

  .badge {
    font-size: 0.65rem;
    font-weight: 700;
    padding: 0.125rem 0.5rem;
    border-radius: 9999px;
    white-space: nowrap;
  }

  .badge.active { background: #3b82f6; color: white; }
  .badge.downloaded { background: #dcfce7; color: #166534; }
  .badge.downloading { background: #fef3c7; color: #92400e; }
  .badge.missing { background: #f3f4f6; color: #6b7280; }

  .description {
    font-size: 0.75rem;
    color: #4b5563;
    line-height: 1.5;
    margin: 0 0 0.5rem 0;
  }

  .meta {
    display: flex;
    gap: 0.5rem;
  }

  .size-tag {
    font-size: 0.65rem;
    font-weight: 600;
    color: #9ca3af;
    background: #f9fafb;
    padding: 0.1rem 0.4rem;
    border-radius: 0.25rem;
    border: 1px solid #f3f4f6;
  }

  .action-group {
    flex-shrink: 0;
  }

  .btn {
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.5rem 0.875rem;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    border: 1px solid transparent;
  }

  .btn-primary {
    background: #ffffff;
    border-color: #d1d5db;
    color: #374151;
  }

  .btn-primary:hover {
    background: #f9fafb;
    border-color: #9ca3af;
  }

  .btn-download {
    background: #111827;
    color: #ffffff;
  }

  .btn-download:hover {
    background: #000000;
    transform: translateY(-1px);
  }

  .btn-secondary {
    background: #f3f4f6;
    border-color: #e5e7eb;
    color: #4b5563;
  }

  .btn-secondary:hover {
    background: #e5e7eb;
    color: #1f2937;
  }

  .btn-small {
    padding: 0.35rem 0.65rem;
    font-size: 0.7rem;
    border-radius: 0.4rem;
  }

  .icon { width: 0.9rem; height: 0.9rem; }

  .check-icon {
    width: 2rem;
    height: 2rem;
    background: #3b82f6;
    color: white;
    border-radius: 9999px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.4rem;
  }

  /* Progress area */
  .progress-ring {
    width: 2.5rem;
    height: 2.5rem;
  }

  .circular-chart {
    display: block;
    margin: 0 auto;
    max-width: 100%;
    max-height: 100%;
  }

  .circle-bg {
    fill: none;
    stroke: #eee;
    stroke-width: 3.8;
  }

  .circle {
    fill: none;
    stroke-width: 3.8;
    stroke-linecap: round;
    stroke: #3b82f6;
    transition: stroke-dasharray 0.3s ease;
  }

  .percentage {
    fill: #3b82f6;
    font-size: 0.5rem;
    font-weight: bold;
    text-anchor: middle;
  }

  .progress-footer {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px dashed #e5e7eb;
  }

  .progress-bar-container {
    height: 4px;
    background: #f3f4f6;
    border-radius: 9999px;
    overflow: hidden;
    margin-bottom: 0.4rem;
  }

  .progress-bar-fill {
    height: 100%;
    background: #3b82f6;
    transition: width 0.3s ease;
  }

  .progress-stats {
    display: flex;
    justify-content: space-between;
    font-size: 0.65rem;
    color: #9ca3af;
    font-weight: 500;
  }

  .speed { color: #3b82f6; }

  .error-panel {
    margin-top: 0.75rem;
    background: #fef2f2;
    border: 1px solid #fee2e2;
    border-radius: 0.5rem;
    padding: 0.5rem 0.75rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .error-text {
    font-size: 0.7rem;
    color: #b91c1c;
    font-weight: 500;
  }

  .retry-link {
    background: none;
    border: none;
    color: #ef4444;
    font-size: 0.7rem;
    font-weight: 700;
    text-decoration: underline;
    cursor: pointer;
    padding: 0;
  }
</style>
