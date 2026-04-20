<script setup lang="ts">
import { browserIconSrc, configurationIconSrc } from "../../utils/icons";
import type { AppPage, BrowserView } from "../../types/browser";

const props = defineProps<{
  browsers: BrowserView[];
  currentBrowserId: string | null;
  page: AppPage;
  loading: boolean;
  configsLoading: boolean;
  browserMonogram: (browserId: string) => string;
  appVersion: string;
}>();

const emit = defineEmits<{
  selectBrowser: [browserId: string];
  selectConfiguration: [];
  refresh: [];
}>();
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-toolbar">
      <div class="sidebar-title-group">
        <h1>浏览器助手</h1>
      </div>
      <button class="refresh-icon-button" type="button" @click="emit('refresh')">
        <svg class="refresh-icon" viewBox="0 0 24 24" aria-hidden="true">
          <path
            d="M20 11a8 8 0 1 0-2.34 5.66M20 4v7h-7"
            fill="none"
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
          />
        </svg>
        <span class="sr-only">{{ loading || configsLoading ? "刷新中..." : "刷新" }}</span>
      </button>
    </div>

    <div v-if="browsers.length" class="browser-nav">
      <button
        v-for="browser in browsers"
        :key="browser.browserId"
        class="browser-nav-item"
        :class="[browser.browserFamilyId ?? browser.browserId, { active: browser.browserId === currentBrowserId && page === 'browserData' }]"
        type="button"
        @click="emit('selectBrowser', browser.browserId)"
      >
        <div class="browser-nav-icon">
          <img
            v-if="browserIconSrc(browser.iconKey ?? browser.browserFamilyId)"
            :src="browserIconSrc(browser.iconKey ?? browser.browserFamilyId) ?? undefined"
            :alt="`${browser.browserName} icon`"
          />
          <span v-else>{{ browserMonogram(browser.browserId) }}</span>
        </div>
        <div class="browser-nav-body">
          <strong>{{ browser.browserName }}</strong>
        </div>
      </button>
    </div>

    <div v-else class="sidebar-empty">
      <p>空空如也。</p>
    </div>

    <button
      class="browser-nav-item utility sidebar-utility-nav"
      :class="{ active: page === 'configuration' }"
      type="button"
      @click="emit('selectConfiguration')"
    >
      <div class="browser-nav-icon config-nav-icon">
        <img :src="configurationIconSrc" alt="配置图标" />
      </div>
      <div class="browser-nav-body">
        <strong>配置</strong>
      </div>
      <span class="utility-version" :title="`版本 ${props.appVersion}`">v{{ props.appVersion }}</span>
    </button>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px 14px;
  min-height: 0;
  border: 1px solid var(--panel-border);
  border-radius: 22px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.92), rgba(248, 250, 252, 0.7));
  box-shadow: var(--shadow);
}

.sidebar-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 2px 2px 8px;
}

.sidebar-title-group {
  min-width: 0;
}

.sidebar-title-group h1 {
  margin: 0;
  font-size: 1.34rem;
  line-height: 1.04;
  font-weight: 600;
  letter-spacing: -0.04em;
}

.refresh-icon-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: 1px solid rgba(148, 163, 184, 0.24);
  border-radius: 12px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.96), rgba(241, 245, 249, 0.92));
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.7);
  cursor: pointer;
  transition:
    transform 160ms ease,
    border-color 160ms ease,
    box-shadow 160ms ease;
}

.refresh-icon-button:hover {
  transform: translateY(-1px);
  border-color: rgba(100, 116, 139, 0.36);
}

.refresh-icon {
  width: 16px;
  height: 16px;
  color: #334155;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.browser-nav {
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-height: 0;
  overflow: auto;
  padding: 2px 2px 0 0;
  scrollbar-width: thin;
  scrollbar-color: rgba(100, 116, 139, 0.42) transparent;
}

.browser-nav::-webkit-scrollbar {
  width: 10px;
}

.browser-nav::-webkit-scrollbar-track {
  background: transparent;
}

.browser-nav::-webkit-scrollbar-thumb {
  border: 3px solid transparent;
  border-radius: 999px;
  background: linear-gradient(180deg, rgba(148, 163, 184, 0.72), rgba(100, 116, 139, 0.58));
  background-clip: padding-box;
}

.browser-nav::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, rgba(100, 116, 139, 0.82), rgba(71, 85, 105, 0.72));
  background-clip: padding-box;
}

.browser-nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 9px 10px;
  border-radius: 14px;
  text-align: left;
  cursor: pointer;
  background: rgba(255, 255, 255, 0.54);
  border: 1px solid transparent;
  transition:
    transform 160ms ease,
    border-color 160ms ease,
    background 160ms ease;
}

.browser-nav-item:hover {
  transform: translateY(-1px);
  border-color: var(--panel-border);
}

.browser-nav-item.active {
  background: var(--accent-soft);
  border-color: rgba(47, 111, 237, 0.18);
}

.browser-nav-item.chrome.active {
  background: rgba(37, 99, 235, 0.12);
}

.browser-nav-item.edge.active {
  background: rgba(8, 145, 178, 0.12);
}

.browser-nav-item.brave.active {
  background: rgba(234, 88, 12, 0.12);
}

.browser-nav-item.utility.active {
  background: rgba(15, 23, 42, 0.08);
  border-color: rgba(15, 23, 42, 0.1);
}

.browser-nav-icon {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  width: 34px;
  height: 34px;
  color: #fff;
  font-weight: 700;
  font-size: 0.78rem;
  letter-spacing: 0.08em;
  background: transparent;
  overflow: visible;
}

.browser-nav-icon img {
  display: block;
  width: auto;
  height: auto;
  max-width: 30px;
  max-height: 30px;
  object-fit: contain;
}

.sidebar-utility-nav {
  margin-top: auto;
  position: relative;
  padding-right: 70px;
}

.browser-nav-body {
  min-width: 0;
}

.browser-nav-body strong {
  display: block;
  color: var(--text);
  font-size: 0.94rem;
  line-height: 1.25;
}

.utility-version {
  position: absolute;
  right: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: rgba(82, 98, 119, 0.7);
  font-size: 0.72rem;
  font-weight: 500;
  letter-spacing: 0.01em;
  pointer-events: none;
}

.sidebar-empty {
  padding: 14px;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.58);
  border: 1px dashed rgba(148, 163, 184, 0.35);
}

.sidebar-empty p {
  margin: 0;
  color: var(--muted);
}
</style>
