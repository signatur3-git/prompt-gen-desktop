/**
 * Keyboard Shortcuts Composable
 *
 * Provides global keyboard shortcut handling for the application
 */

import { onMounted, onUnmounted } from 'vue';

export interface ShortcutHandler {
  key: string;
  ctrl?: boolean;
  alt?: boolean;
  shift?: boolean;
  handler: () => void | Promise<void> | Promise<any>;
  description: string;
}

export function useKeyboardShortcuts(shortcuts: ShortcutHandler[]) {
  const handleKeyDown = async (event: KeyboardEvent) => {
    for (const shortcut of shortcuts) {
      const keyMatch = event.key.toLowerCase() === shortcut.key.toLowerCase();
      const ctrlMatch = shortcut.ctrl === undefined || event.ctrlKey === shortcut.ctrl;
      const altMatch = shortcut.alt === undefined || event.altKey === shortcut.alt;
      const shiftMatch = shortcut.shift === undefined || event.shiftKey === shortcut.shift;

      if (keyMatch && ctrlMatch && altMatch && shiftMatch) {
        event.preventDefault();
        await shortcut.handler();
        break;
      }
    }
  };

  onMounted(() => {
    window.addEventListener('keydown', handleKeyDown);
  });

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });

  return {
    shortcuts,
  };
}

/**
 * Get formatted shortcut string for display
 */
export function formatShortcut(shortcut: ShortcutHandler): string {
  const parts: string[] = [];
  if (shortcut.ctrl) parts.push('Ctrl');
  if (shortcut.alt) parts.push('Alt');
  if (shortcut.shift) parts.push('Shift');
  parts.push(shortcut.key.toUpperCase());
  return parts.join('+');
}

