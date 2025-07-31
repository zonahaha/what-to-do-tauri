<template>
  <transition-group name="todo-move" tag="div">
    <div class="todo-list" ref="todoListRef">
      <div
        v-for="(todo, index) in todos"
        :key="todo.id"
        class="todo-item"
        :class="{ completed: todo.completed }"
        :style="{
          animationDelay: index * 0.1 + 's',
          '--border-color': getBorderColor(index),
        }"
      >
        <div class="todo-content">
          <label class="checkbox-container">
            <input
              type="checkbox"
              :checked="todo.completed"
              @change="$emit('toggle-todo', todo.id)"
              class="todo-checkbox"
            />
            <span class="checkmark"></span>
          </label>
          <div class="todo-text-container">
            <span class="todo-text">{{ todo.text }}</span>
            <span class="todo-time">{{ formatTime(todo.id) }}</span>
          </div>
        </div>
        <button
          @click="$emit('delete-todo', todo.id)"
          class="delete-btn"
          title="删除"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"
            />
          </svg>
        </button>
      </div>

      <div v-if="todos.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg
            width="80"
            height="80"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
          >
            <path d="M9 11H1l3-3-3-3" />
            <path d="M23 20v-6a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v6" />
          </svg>
        </div>
        <h3>开始你的计划</h3>
        <p>添加一些任务，让每一天都充满成就感</p>
      </div>
    </div>
  </transition-group>
</template>

<script>
export default {
  name: 'TodoList',
  props: {
    todos: {
      type: Array,
      required: true,
    },
  },
  data() {
    return {
      // 定义不同的边框颜色
      borderColors: [
        'rgba(139, 92, 246, 0.6)', // 紫色
        'rgba(59, 130, 246, 0.6)', // 蓝色
        'rgba(16, 185, 129, 0.6)', // 绿色
        'rgba(245, 158, 11, 0.6)', // 橙色
        'rgba(239, 68, 68, 0.6)', // 红色
        'rgba(168, 85, 247, 0.6)', // 深紫色
        'rgba(6, 182, 212, 0.6)', // 青色
        'rgba(251, 113, 133, 0.6)', // 粉色
        'rgba(34, 197, 94, 0.6)', // 深绿色
        'rgba(249, 115, 22, 0.6)', // 深橙色
      ],
    };
  },
  methods: {
    formatTime(timestamp) {
      const date = new Date(timestamp);
      const now = new Date();
      const diff = now - date;
      const minutes = Math.floor(diff / 60000);
      const hours = Math.floor(diff / 3600000);
      const days = Math.floor(diff / 86400000);

      if (minutes < 1) return '刚刚';
      if (minutes < 60) return `${minutes}分钟前`;
      if (hours < 24) return `${hours}小时前`;
      if (days < 7) return `${days}天前`;
      return date.toLocaleDateString();
    },
    getBorderColor(index) {
      // 循环使用颜色数组
      return this.borderColors[index % this.borderColors.length];
    },
  },
};
</script>

<style scoped>
.todo-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-height: 0; /* 确保可以收缩 */
  width: 100%; /* 确保占满父容器宽度 */
  min-width: 0; /* 允许收缩但保持最小宽度 */
  max-width: 100%; /* 确保不超过容器宽度 */
  box-sizing: border-box; /* 确保padding和border不会影响总宽度 */
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  text-align: center;
  color: #a5b4fc;
}

.todo-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-radius: 16px;
  border: 2px solid var(--border-color, rgba(255, 255, 255, 0.1));
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3), 0 2px 8px rgba(0, 0, 0, 0.2);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  animation: slideIn 0.5s ease-out forwards;
  opacity: 0;
  transform: translateY(20px);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  min-height: 72px; /* 设置最小高度，确保固定高度 */
  flex-shrink: 0; /* 防止收缩 */
  width: 100%; /* 确保占满父容器宽度 */
  min-width: 0; /* 允许收缩但保持最小宽度 */
  box-sizing: border-box; /* 确保padding和border不会影响总宽度 */
}

.todo-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: var(
    --border-color,
    linear-gradient(90deg, #8b5cf6 0%, #a855f7 50%, #d946ef 100%)
  );
  opacity: 0;
  transition: opacity 0.3s ease;
}

.todo-item:hover::before {
  opacity: 1;
}

.todo-item:hover {
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4), 0 4px 16px rgba(0, 0, 0, 0.3);
  border-color: var(--border-color, rgba(255, 255, 255, 0.2));
  filter: brightness(1.1);
}

.todo-content {
  display: flex;
  align-items: center;
  flex: 1;
  min-height: 0; /* 确保内容不会撑大容器 */
}

.checkbox-container {
  position: relative;
  display: inline-block;
  width: 24px;
  height: 24px;
  margin-right: 16px;
  cursor: pointer;
}

.todo-checkbox {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

.checkmark {
  position: absolute;
  top: 0;
  left: 0;
  width: 24px;
  height: 24px;
  background: rgba(255, 255, 255, 0.1);
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 8px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.checkbox-container:hover .checkmark {
  border-color: var(--primary-color, #8b5cf6);
  box-shadow: 0 4px 12px rgba(var(--primary-color-rgb, 139, 92, 246), 0.3);
}

.todo-checkbox:checked ~ .checkmark {
  background: linear-gradient(
    135deg,
    var(--primary-color, #8b5cf6) 0%,
    var(--secondary-color, #a855f7) 50%,
    var(--secondary-color, #d946ef) 100%
  );
  border-color: var(--primary-color, #8b5cf6);
  box-shadow: 0 4px 16px rgba(var(--primary-color-rgb, 139, 92, 246), 0.4);
}

.checkmark:after {
  content: '';
  position: absolute;
  display: none;
  left: 8px;
  top: 4px;
  width: 4px;
  height: 10px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.todo-checkbox:checked ~ .checkmark:after {
  display: block;
}

.todo-text-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.todo-text {
  font-size: 16px;
  color: #fff;
  line-height: 1.5;
  font-weight: 500;
  transition: all 0.3s ease;
}

.todo-time {
  font-size: 12px;
  color: #a5b4fc;
  font-weight: 400;
}

.completed .todo-text {
  text-decoration: line-through;
  color: #6b7280;
  font-weight: 400;
}

.completed .todo-time {
  color: #4b5563;
}

.delete-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: rgba(0, 0, 0, 0.1) !important;
  color: #f87171;
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  opacity: 0.7;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.delete-btn:hover {
  background: rgba(239, 68, 68, 0.05);
  opacity: 1;
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.2);
}

.delete-btn:active {
  transform: scale(0.95);
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  text-align: center;
  color: #a5b4fc;
}

.empty-icon {
  margin-bottom: 32px;
  opacity: 0.3;
  animation: float 3s ease-in-out infinite;
}

.empty-state h3 {
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 12px;
  color: #fff;
  background: linear-gradient(
    135deg,
    var(--primary-color, #8b5cf6) 0%,
    var(--secondary-color, #a855f7) 50%,
    var(--secondary-color, #d946ef) 100%
  );
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.empty-state p {
  font-size: 16px;
  opacity: 0.7;
  line-height: 1.5;
  max-width: 280px;
}

@keyframes slideIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-10px);
  }
}

/* 列表项移动动画 */
.todo-move-move {
  transition: transform 0.5s ease;
}

.todo-move-enter-active,
.todo-move-leave-active {
  transition: all 0.5s ease;
}

.todo-move-enter-from {
  opacity: 0;
  transform: translateY(-20px);
}

.todo-move-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}
</style>
