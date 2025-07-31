<template>
  <div class="add-todo">
    <div class="input-wrapper">
      <div class="input-container">
        <div class="input-icon">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M5 12h14"/>
          </svg>
        </div>
        <input 
          v-model="newTodo" 
          @keyup.enter="handleAdd"
          placeholder="添加新的任务..."
          class="todo-input"
        />
        <button @click="handleAdd" class="add-btn" :disabled="!newTodo.trim()">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5v14"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'AddTodo',
  props: {
    currentTheme: {
      type: Object,
      default: ()=>({
        name: '深紫夜',
          primary: '#6366f1',
          secondary: '#8b5cf6',
          bgColors: [
            'rgba(120, 119, 198, 0.3)',
            'rgba(255, 119, 198, 0.3)',
            'rgba(120, 219, 255, 0.3)'
          ]
      })
    }
  },
  computed: {
    btnColor(){
      return `linear-gradient(135deg, ${this.currentTheme.bgColors[0]} 0%, ${this.currentTheme.bgColors[1]} 50%, ${this.currentTheme.bgColors[2]} 100%)`
    }
  },
  data() {
    return {
      newTodo: ''
    }
  },
  methods: {
    handleAdd() {
      if (this.newTodo.trim()) {
        this.$emit('add-todo', this.newTodo.trim())
        this.newTodo = ''
      }
    }
  }
}
</script>

<style scoped>
.add-todo {
  margin-bottom: 24px;
  width: 100%;
  box-sizing: border-box;
}

.input-wrapper {
  position: relative;
  width: 100%;
  box-sizing: border-box;
}

.input-container {
  display: flex;
  align-items: center;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 
    0 8px 32px rgba(0, 0, 0, 0.3),
    0 4px 16px rgba(0, 0, 0, 0.2);
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  width: 100%;
  box-sizing: border-box;
}

.input-container:focus-within {
  border-color: rgba(139, 92, 246, 0.5);
  box-shadow: 
    0 0 0 4px rgba(139, 92, 246, 0.1),
    0 12px 40px rgba(0, 0, 0, 0.4),
    0 6px 20px rgba(0, 0, 0, 0.3);
  transform: translateY(-2px);
}

.input-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  color: #a5b4fc;
  background: rgba(255, 255, 255, 0.05);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
}

.todo-input {
  flex: 1;
  padding: 16px 20px;
  border: none;
  background: transparent;
  font-size: 16px;
  color: #fff;
  outline: none;
  font-family: inherit;
  font-weight: 500;
}

.todo-input::placeholder {
  color: #a5b4fc;
  font-weight: 400;
}

.add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background: v-bind(btnColor);
  color: white;
  border: none;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.add-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.6s;
}

.add-btn:hover:not(:disabled)::before {
  left: 100%;
}

.add-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, #7c3aed 0%, #9333ea 50%, #c026d3 100%);
  transform: translateY(-2px);
  box-shadow: 
    0 12px 32px rgba(139, 92, 246, 0.4),
    0 6px 16px rgba(139, 92, 246, 0.3);
}

.add-btn:active:not(:disabled) {
  transform: translateY(0);
  box-shadow: 
    0 6px 16px rgba(139, 92, 246, 0.3),
    0 3px 8px rgba(139, 92, 246, 0.2);
}

.add-btn:disabled {
  background: linear-gradient(135deg, #374151 0%, #4b5563 100%);
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
  opacity: 0.5;
}

.add-btn svg {
  transition: transform 0.3s ease;
  position: relative;
  z-index: 1;
}

.add-btn:hover:not(:disabled) svg {
  transform: scale(1.1) rotate(90deg);
}
</style>