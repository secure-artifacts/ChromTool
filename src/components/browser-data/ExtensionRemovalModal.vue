<script setup lang="ts">
import { computed } from "vue";

import type { ExtensionSummary, ProfileSummary, RemoveExtensionResult } from "../../types/browser";

const props = defineProps<{
  mode: "confirm" | "result";
  title: string;
  extensions: ExtensionSummary[];
  profiles: ProfileSummary[];
  results: RemoveExtensionResult[];
  busy?: boolean;
  generalError?: string;
}>();

const emit = defineEmits<{
  close: [];
  confirm: [];
}>();

const confirmSummary = computed(() => ({
  extensionCount: props.extensions.length,
  profileCount: props.profiles.length,
}));

const resultSummary = computed(() => {
  const statusByExtension = new Map<string, boolean>();

  for (const result of props.results) {
    const previous = statusByExtension.get(result.extensionId);
    const succeeded = !result.error;

    if (previous === undefined) {
      statusByExtension.set(result.extensionId, succeeded);
      continue;
    }

    statusByExtension.set(result.extensionId, previous && succeeded);
  }

  let successCount = 0;
  let failedCount = 0;

  for (const succeeded of statusByExtension.values()) {
    if (succeeded) {
      successCount += 1;
    } else {
      failedCount += 1;
    }
  }

  return { successCount, failedCount };
});
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <section class="modal-card">
      <div class="modal-header">
        <h3>{{ title }}</h3>
        <button class="secondary-button" type="button" @click="emit('close')">关闭</button>
      </div>

      <template v-if="mode === 'confirm'">
        <p class="modal-copy">
          将从 {{ confirmSummary.profileCount }} 个资料中删除 {{ confirmSummary.extensionCount }} 个插件。注意：你需要关闭所有浏览器。
        </p>

        <div class="modal-actions">
          <button class="secondary-button" type="button" @click="emit('close')">取消</button>
          <button class="danger-button" type="button" :disabled="busy" @click="emit('confirm')">
            {{ busy ? "删除中..." : "确认删除" }}
          </button>
        </div>
      </template>

      <template v-else>
        <p v-if="generalError" class="result-banner error">{{ generalError }}</p>
        <p class="modal-copy">
          成功删除 {{ resultSummary.successCount }} 个插件，失败 {{ resultSummary.failedCount }} 个。
        </p>

        <div class="modal-actions">
          <button class="primary-button" type="button" @click="emit('close')">关闭</button>
        </div>
      </template>
    </section>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(15, 23, 42, 0.26);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
}

.modal-card {
  width: min(680px, 100%);
  max-height: min(76vh, 820px);
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--panel-border);
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.96);
  box-shadow: 0 28px 70px rgba(15, 23, 42, 0.18);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.modal-header h3,
.modal-copy {
  margin: 0;
}

.modal-copy {
  color: var(--muted);
  line-height: 1.55;
}

.result-banner {
  margin: 0;
  padding: 12px 14px;
  border-radius: 14px;
  font-size: 0.9rem;
}

.result-banner.error {
  background: rgba(254, 242, 242, 0.96);
  color: #b42318;
  border: 1px solid rgba(239, 68, 68, 0.18);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.muted-line {
  color: var(--muted-soft);
}

code {
  padding: 1px 5px;
  border-radius: 7px;
  background: rgba(226, 232, 240, 0.72);
  color: var(--text);
}
</style>
