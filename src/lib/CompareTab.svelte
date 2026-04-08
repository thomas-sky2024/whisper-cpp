<script lang="ts">
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import pkg from "diff-match-patch";
  const { diff_match_patch: DiffMatchPatch, DIFF_DELETE, DIFF_INSERT, DIFF_EQUAL } = pkg;
  import { jobStore, generateSRTContent } from "$lib/jobStore";
  import { onMount } from "svelte";

  // ── State ─────────────────────────────────────────────────────────────
  let textOriginal = $state("");
  let textAI = $state("");
  let textResult = $state("");

  type Token = { type: number; text: string };
  let diffTokens = $state<Token[]>([]);
  let activeView = $state<"diff" | "result">("result");
  let matches = $state({ total: 0, matched: 0, percent: 0 });

  // ── Auto-fill khi mở tab ──────────────────────────────────────────────
  onMount(() => {
    if ($jobStore.segments && $jobStore.segments.length > 0) {
      textAI = generateSRTContent($jobStore.segments);
    }
  });

  // ── File loaders ──────────────────────────────────────────────────────
  async function loadFile(target: "original" | "ai") {
    const filePath = await open({
      multiple: false,
      filters: [{ name: "Text/SRT", extensions: ["txt", "srt"] }],
    });
    if (filePath && typeof filePath === "string") {
      const content = await readTextFile(filePath);
      if (target === "original") textOriginal = content;
      else textAI = content;
    }
  }

  // ── Helpers ───────────────────────────────────────────────────────────
  function cleanCN(s: string) {
    return (s || "").replace(/[^\u4e00-\u9fa5]/g, "");
  }

  // ── Logic 1: Diff so sánh ký tự Hán ──────────────────────────────────
  // diff_main(a, b) → tính sự khác biệt từ a sang b
  // Chúng ta muốn: INSERT = có trong Gốc nhưng thiếu ở AI, DELETE = AI thừa/sai
  // Vậy diff_main(aiText, refText): INSERT = thêm vào để ra refText = có trong gốc thiếu ở AI ✓
  function runDiff() {
    const refText = cleanCN(textOriginal);
    const aiText = cleanCN(textAI);

    if (!refText || !aiText) {
      alert("Cần có ký tự tiếng Trung ở cả hai ô!");
      return;
    }

    const dmp = new DiffMatchPatch();
    const diffs = dmp.diff_main(aiText, refText);
    dmp.diff_cleanupSemantic(diffs);

    diffTokens = diffs.map(([type, text]) => ({ type, text }));

    let matched = 0;
    diffs.forEach(([op, txt]) => {
      if (op === DIFF_EQUAL) matched += txt.length;
    });

    matches = {
      total: refText.length,
      matched,
      percent: refText.length > 0 ? (matched / refText.length) * 100 : 0,
    };

    activeView = "diff";
  }

  // ── Logic 2: Đồng bộ SRT — ghép chữ Hán từ Gốc vào Timestamp AI ─────
  // Thuật toán: đếm ký tự Hán trong mỗi block AI → lấy đúng số đó từ Gốc → thay thế
  // Giữ nguyên dấu câu / khoảng trắng của AI
  function syncSRT() {
    const refChars = Array.from(cleanCN(textOriginal));
    const blocks = textAI.trim().split(/\n\s*\n/);
    let ci = 0;

    const outBlocks = blocks.map((block) => {
      const lines = block.split("\n");
      const tsIdx = lines.findIndex((l) => l.includes("-->"));
      if (tsIdx === -1) return block;

      const subLines = lines.slice(tsIdx + 1);
      const aiCNCount = (subLines.join("").match(/[\u4e00-\u9fa5]/g) || []).length;

      // Lấy đúng aiCNCount ký tự từ refChars
      let newChars = "";
      for (let k = 0; k < aiCNCount && ci < refChars.length; k++, ci++) {
        newChars += refChars[ci];
      }

      // Thay ký tự Hán, giữ nguyên dấu câu/số/latin
      let nci = 0;
      const newSubLines = subLines.map((line) => {
        let nl = "";
        for (const ch of line) {
          if (/[\u4e00-\u9fa5]/.test(ch)) {
            nl += nci < newChars.length ? newChars[nci++] : ch;
          } else {
            nl += ch;
          }
        }
        return nl;
      });

      return [...lines.slice(0, tsIdx + 1), ...newSubLines].join("\n");
    });

    textResult = outBlocks.join("\n\n");
    activeView = "result";
  }

  // ── Export ────────────────────────────────────────────────────────────
  async function downloadSRT() {
    if (!textResult) return;
    const path = await save({
      filters: [{ name: "SRT", extensions: ["srt"] }],
      defaultPath: "synced_result.srt",
    });
    if (path) await writeTextFile(path, textResult);
  }

  // Download toàn bộ textOriginal (raw), không strip ký tự
  function downloadTXT() {
    const blob = new Blob([textOriginal], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "original_script.txt";
    a.click();
    URL.revokeObjectURL(url);
  }

  // Tỷ lệ màu cho badge
  const pctColor = $derived(
    matches.percent >= 90 ? "#34d399"
    : matches.percent >= 70 ? "#fbbf24"
    : "#f87171"
  );
</script>

<div class="wrap">

  <!-- ── Toolbar ── -->
  <div class="toolbar">
    <button class="btn btn-ghost" onclick={() => openUrl("https://www.purpleculture.net/traditional-simplified-converter/")}>
      🌐 Giản/Phồn
    </button>

    <div class="toolbar-right">
      <button class="btn btn-secondary" onclick={runDiff} disabled={!textOriginal || !textAI}>
        🔍 So sánh
      </button>
      <button class="btn btn-primary" onclick={syncSRT} disabled={!textOriginal || !textAI}>
        ✨ Đồng bộ SRT
      </button>
    </div>
  </div>

  <!-- ── Input panels ── -->
  <div class="inputs">
    <div class="panel">
      <div class="panel-head">
        <span class="panel-label">Kịch bản gốc</span>
        <button class="link-btn" onclick={() => loadFile("original")}>Tải .txt</button>
      </div>
      <textarea
        class="ta"
        bind:value={textOriginal}
        placeholder="Dán kịch bản gốc vào đây…"
      ></textarea>
    </div>

    <div class="panel">
      <div class="panel-head">
        <span class="panel-label">Bản AI / Whisper</span>
        <button class="link-btn" onclick={() => loadFile("ai")}>Tải .srt</button>
      </div>
      <textarea
        class="ta"
        bind:value={textAI}
        placeholder="SRT từ Whisper sẽ tự điền sau khi transcribe…"
      ></textarea>
    </div>
  </div>

  <!-- ── Result panel ── -->
  <div class="result-wrap">
    <div class="result-head">
      <div class="result-title-row">
        <span class="result-label">
          {activeView === "diff" ? "So sánh" : "Kết quả SRT"}
        </span>

        {#if activeView === "diff" && matches.total > 0}
          <span class="badge" style="--c: {pctColor}">
            <span class="badge-pct">{Math.round(matches.percent)}%</span>
            <span class="badge-detail">{matches.matched}/{matches.total}</span>
          </span>
        {/if}
      </div>

      {#if activeView === "result" && textResult}
        <div class="result-actions">
          <button class="btn btn-sm btn-ghost" onclick={downloadTXT}>⬇ TXT</button>
          <button class="btn btn-sm btn-primary" onclick={downloadSRT}>⬇ SRT</button>
        </div>
      {/if}
    </div>

    <div class="result-body">
      {#if activeView === "diff"}
        {#if diffTokens.length > 0}
          <div class="diff-view">
            {#each diffTokens as token}
              {#if token.type === DIFF_EQUAL}
                <span class="t-eq">{token.text}</span>
              {:else if token.type === DIFF_INSERT}
                <span class="t-ins" title="Có trong gốc, thiếu ở AI">{token.text}</span>
              {:else if token.type === DIFF_DELETE}
                <span class="t-del" title="AI nghe thừa/sai">{token.text}</span>
              {/if}
            {/each}
          </div>
        {:else}
          <div class="empty">Nhấn "So sánh" để xem sự khác biệt.</div>
        {/if}

      {:else}
        {#if textResult}
          <pre class="srt-pre">{textResult}</pre>
        {:else}
          <div class="empty">
            <div class="empty-icon">✨</div>
            <p>Nhấn "Đồng bộ SRT" để ghép chữ từ bản gốc vào timestamp của AI.</p>
          </div>
        {/if}
      {/if}
    </div>
  </div>

</div>

<style>
  .wrap {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    height: 100%;
    padding: 1rem 1.25rem;
    overflow: hidden;
    font-family: 'Inter', -apple-system, sans-serif;
    animation: fadeIn 0.25s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(6px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  /* ── Toolbar ── */
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: #13141c;
    border: 1px solid #2a2d3e;
    border-radius: 10px;
    padding: 0.5rem 0.75rem;
    flex-shrink: 0;
  }

  .toolbar-right {
    display: flex;
    gap: 0.5rem;
  }

  /* ── Inputs ── */
  .inputs {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
    flex: 0 0 28%;
    min-height: 180px;
  }

  .panel {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    min-width: 0;
  }

  .panel-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 2px;
  }

  .panel-label {
    font-size: 0.7rem;
    font-weight: 700;
    color: #555a7a;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .link-btn {
    font-size: 0.7rem;
    color: #7c8cf8;
    background: none;
    border: none;
    cursor: pointer;
    font-weight: 600;
    padding: 0;
  }
  .link-btn:hover { text-decoration: underline; }

  .ta {
    flex: 1;
    width: 100%;
    background: #1a1b28;
    border: 1px solid #2a2d3e;
    border-radius: 8px;
    padding: 0.75rem;
    color: #c4c8e2;
    font-size: 0.95rem;
    font-family: 'Songti SC', 'SimSun', serif;
    line-height: 1.6;
    resize: none;
    outline: none;
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  .ta:focus {
    border-color: #7c8cf8;
    box-shadow: 0 0 0 2px rgba(124,140,248,0.15);
  }

  /* ── Result ── */
  .result-wrap {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    background: #0e1016;
    border: 1px solid #2a2d3e;
    border-radius: 10px;
    overflow: hidden;
  }

  .result-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.85rem;
    border-bottom: 1px solid #2a2d3e;
    background: #13141c;
    flex-shrink: 0;
  }

  .result-title-row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .result-label {
    font-size: 0.7rem;
    font-weight: 700;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .badge {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    background: rgba(255,255,255,0.04);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 999px;
    padding: 0.1rem 0.5rem;
  }
  .badge-pct {
    color: var(--c, #7c8cf8);
    font-weight: 800;
    font-size: 0.72rem;
  }
  .badge-detail {
    color: #475569;
    font-size: 0.67rem;
  }

  .result-actions { display: flex; gap: 0.4rem; }

  .result-body {
    flex: 1;
    overflow-y: auto;
    padding: 1rem 1.25rem;
  }

  /* ── Diff ── */
  .diff-view {
    font-family: 'Songti SC', 'SimSun', serif;
    font-size: 1.15rem;
    line-height: 1.9;
    color: #8b92b8;
    word-break: break-all;
  }
  .t-eq  { opacity: 0.75; }
  .t-ins {
    background: rgba(16,185,129,0.12);
    color: #34d399;
    border-bottom: 2px solid rgba(16,185,129,0.3);
    border-radius: 2px;
    padding: 0 1px;
  }
  .t-del {
    background: rgba(239,68,68,0.12);
    color: #f87171;
    text-decoration: line-through;
    opacity: 0.65;
    border-radius: 2px;
    padding: 0 1px;
  }

  .srt-pre {
    color: #e2e8f0;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.82rem;
    line-height: 1.7;
    white-space: pre-wrap;
    margin: 0;
  }

  /* ── Empty ── */
  .empty {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    color: #3d4466;
    gap: 0.4rem;
    font-size: 0.875rem;
  }
  .empty-icon { font-size: 2rem; margin-bottom: 0.5rem; }

  /* ── Buttons ── */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.45rem 0.9rem;
    border-radius: 7px;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.15s ease;
    white-space: nowrap;
  }
  .btn:disabled { opacity: 0.35; cursor: not-allowed; filter: grayscale(1); }

  .btn-primary {
    background: linear-gradient(135deg, #7c8cf8, #a78bfa);
    color: #fff;
    box-shadow: 0 3px 10px rgba(124,140,248,0.25);
  }
  .btn-primary:hover:not(:disabled) {
    filter: brightness(1.1);
    box-shadow: 0 4px 14px rgba(124,140,248,0.4);
  }

  .btn-secondary {
    background: #252840;
    color: #a5b4fc;
    border: 1px solid #3a3e5c;
  }
  .btn-secondary:hover:not(:disabled) { background: #2d3254; border-color: #4c5277; }

  .btn-ghost {
    background: transparent;
    color: #8892b0;
    border: 1px solid #2a2d3e;
  }
  .btn-ghost:hover:not(:disabled) { background: rgba(255,255,255,0.04); color: #c4c8e2; }

  .btn-sm { padding: 0.3rem 0.65rem; font-size: 0.75rem; }

  /* ── Scrollbar ── */
  ::-webkit-scrollbar { width: 6px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: #2a2d3e; border-radius: 6px; }
  ::-webkit-scrollbar-thumb:hover { background: #3a3e5c; }
</style>
