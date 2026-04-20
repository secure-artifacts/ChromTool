<script setup lang="ts">
import { computed, ref } from "vue";
import SortDropdown from "../SortDropdown.vue";
import { browserIconOptions, browserIconSrc } from "../../utils/icons";
import type { BrowserConfigEntry, CreateCustomBrowserConfigInput } from "../../types/browser";

defineProps<{
  configError: string;
  configsLoading: boolean;
  browserConfigs: BrowserConfigEntry[];
  createConfigForm: CreateCustomBrowserConfigInput;
  savingConfig: boolean;
  configMonogram: (config: BrowserConfigEntry) => string;
  isDeletingConfig: (configId: string) => boolean;
}>();

const emit = defineEmits<{
  updateName: [value: string];
  updateExecutablePath: [value: string];
  updateUserDataPath: [value: string];
  updateIconKey: [value: string];
  pickExecutablePath: [];
  pickUserDataPath: [];
  createConfig: [];
  deleteConfig: [configId: string];
}>();

const formExpanded = ref(false);

const iconOptions = computed(() =>
  browserIconOptions.map((option) => ({
    label: option.label,
    value: option.key,
    iconSrc: option.src,
  })),
);
</script>

<template>
  <section class="config-page styled-scrollbar">
    <div v-if="configError" class="inline-error">
      {{ configError }}
    </div>

    <div class="config-form-card">
      <div class="config-form-header collapsible">
        <div>
          <h3>添加自定义浏览器</h3>
        </div>
        <button
          class="secondary-button config-toggle-button"
          type="button"
          @click="formExpanded = !formExpanded"
        >
          {{ formExpanded ? "收起" : "展开" }}
        </button>
      </div>
      <div v-if="formExpanded" class="config-form-fields compact">
        <div class="config-inline-row">
          <label class="field-group">
            <span>名称</span>
            <input
              :value="createConfigForm.name"
              placeholder="例如：工作 Chrome"
              @input="emit('updateName', ($event.target as HTMLInputElement).value)"
            />
          </label>
          <label class="field-group">
            <span>图标</span>
            <SortDropdown
              :model-value="createConfigForm.iconKey ?? 'chrome'"
              :options="iconOptions"
              @update:model-value="emit('updateIconKey', $event)"
            />
          </label>
        </div>
        <label class="field-group">
          <span>可执行文件路径</span>
          <div class="path-input-row">
            <input
              :value="createConfigForm.executablePath"
              placeholder="C:\Program Files\...\chrome.exe"
              @input="emit('updateExecutablePath', ($event.target as HTMLInputElement).value)"
            />
            <button class="secondary-button" type="button" @click="emit('pickExecutablePath')">
              选择文件
            </button>
          </div>
        </label>
        <label class="field-group">
          <span>用户资料路径</span>
          <div class="path-input-row">
            <input
              :value="createConfigForm.userDataPath"
              placeholder="C:\Users\...\User Data"
              @input="emit('updateUserDataPath', ($event.target as HTMLInputElement).value)"
            />
            <button class="secondary-button" type="button" @click="emit('pickUserDataPath')">
              选择文件夹
            </button>
          </div>
        </label>
        <div class="config-form-actions">
          <button
            class="primary-button"
            type="button"
            :disabled="savingConfig"
            @click="emit('createConfig')"
          >
            {{ savingConfig ? "保存中..." : "添加配置" }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="configsLoading" class="empty-card">
      <p>正在加载浏览器配置...</p>
    </div>
    <div v-else class="stack-list">
      <article
        v-for="config in browserConfigs"
        :key="config.id"
        class="config-card"
      >
        <div class="config-card-header">
          <div class="config-card-lead">
            <div class="browser-nav-icon config-icon">
              <img
                v-if="browserIconSrc(config.iconKey ?? config.browserFamilyId)"
                :src="browserIconSrc(config.iconKey ?? config.browserFamilyId) ?? undefined"
                :alt="`${config.name} icon`"
              />
              <span v-else>{{ configMonogram(config) }}</span>
            </div>
            <div>
              <div class="config-title-row">
                <h4>{{ config.name }}</h4>
              </div>
            </div>
          </div>
          <button
            v-if="config.deletable"
            class="danger-button"
            type="button"
            :disabled="isDeletingConfig(config.id)"
            @click="emit('deleteConfig', config.id)"
          >
            {{ isDeletingConfig(config.id) ? "删除中..." : "删除" }}
          </button>
        </div>
        <div class="config-meta">
          <div class="config-meta-row">
            <span class="config-label">可执行文件</span>
            <p :title="config.executablePath">{{ config.executablePath || "未找到" }}</p>
          </div>
          <div class="config-meta-row">
            <span class="config-label">用户资料</span>
            <p :title="config.userDataPath">{{ config.userDataPath }}</p>
          </div>
        </div>
      </article>
    </div>
  </section>
</template>

<style scoped>
.config-page {
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-height: 0;
  overflow: auto;
  padding: 0 2px 0 0;
}

.config-form-card,
.config-card {
  border-radius: 18px;
  padding: 12px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  background: var(--panel-strong);
}

.config-form-header h3,
.config-title-row h4 {
  margin: 0;
  font-size: 1.1rem;
  line-height: 1.2;
}

.config-form-header.collapsible {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.config-meta-row p {
  margin: 6px 0 0;
  color: var(--muted);
  font-size: 0.82rem;
}

.config-form-fields {
  display: grid;
  gap: 10px;
}

.config-form-fields.compact {
  margin-top: 12px;
}

.config-inline-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 240px;
  gap: 12px;
  align-items: end;
}

.field-group {
  display: grid;
  gap: 6px;
}

.field-group span,
.config-label {
  color: var(--muted);
  font-size: 0.8rem;
  font-weight: 600;
}

.field-group input {
  width: 100%;
  padding: 9px 11px;
  border: 1px solid rgba(148, 163, 184, 0.24);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.94);
  color: var(--text);
  outline: none;
}

.field-group input:focus {
  border-color: rgba(47, 111, 237, 0.42);
  box-shadow: 0 0 0 3px rgba(47, 111, 237, 0.12);
}

.field-group :deep(.sort-dropdown) {
  width: 100%;
}

.field-group :deep(.sort-dropdown-trigger) {
  width: 100%;
  min-width: 0;
  min-height: 40px;
}

.path-input-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 118px;
  gap: 10px;
}

.config-toggle-button {
  white-space: nowrap;
}

.path-input-row .secondary-button {
  width: 118px;
  justify-content: center;
}

.config-form-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 2px;
}

.config-card-header,
.config-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.config-card-lead {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  flex: 1;
}

.config-icon {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  width: 30px;
  height: 30px;
  font-size: 0.74rem;
  overflow: visible;
}

.config-icon img {
  display: block;
  width: auto;
  height: auto;
  max-width: 26px;
  max-height: 26px;
  object-fit: contain;
}

.config-meta {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  margin-top: 10px;
}

.config-meta-row {
  display: grid;
  gap: 3px;
  padding: 10px 12px;
  border-radius: 12px;
  background: rgba(248, 250, 252, 0.78);
  border: 1px solid rgba(148, 163, 184, 0.12);
}

.config-meta-row p {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

@media (max-width: 720px) {
  .config-inline-row,
  .config-meta {
    grid-template-columns: 1fr;
  }
}
</style>
