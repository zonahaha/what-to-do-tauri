<template>
  <div id="app">
    <!-- 主题选择器移到顶层，完全脱离布局流 -->
    <div class="theme-selector">
            <!-- 调色盘小框（默认显示） -->
            <div 
              class="theme-toggle" 
              :class="{ 'expanded': showThemeMenu }"
              @click="toggleThemeMenu"
              v-show="!showThemeMenu"
            >
              <img src="./assets/colorSelector.png" alt="颜色选择器" width="20" height="20" />
            </div>
            
            <!-- 展开的主题菜单 -->
            <div class="theme-menu" :class="{ 'expanded': showThemeMenu }">
              <!-- 收起按钮 -->
              <div class="theme-collapse" @click="toggleThemeMenu" v-show="showThemeMenu">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M9 18l6-6-6-6"/>
                </svg>
              </div>
              
              <!-- 主题选项 -->
              <div 
                v-for="theme in themes" 
                :key="theme.name"
                class="theme-option"
                :class="{ active: currentTheme.name === theme.name }"
                @click="selectTheme(theme)"
              >
                <div class="theme-preview" :style="{ background: `linear-gradient(135deg, ${theme.primary}C0 0%, ${theme.secondary}40 100%)` }"></div>
              </div>
            </div>
          </div>
    
        <div class="container">
       <div class="bg"></div>
      <div class="header" :class="{ 'compact': scrollInfo.isScrollingUp && isOverflow }">
       
        <div class="header-content" v-show="!scrollInfo.isScrollingUp || !isOverflow">
          <div class="title-section">
            <h1>我的任务</h1>
            <p class="subtitle">让每一天都充满成就感</p>
          </div>
          <div class="stats-section">
            <div class="stats-card">
              <div class="stats-number">{{ completedCount }}</div>
              <div class="stats-label">已完成</div>
            </div>
            <div class="stats-card">
              <div class="stats-number">{{ todos.length }}</div>
              <div class="stats-label">总任务</div>
            </div>
          </div>
          
        </div>
        <div class="progress-section">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercentage + '%' }"></div>
          </div>
          <div class="progress-text">{{ progressPercentage }}% 完成</div>
        </div>

      </div>
      <div class="content" 
           :class="{ 'no-scroll': todos.length === 0 }" 
           ref="contentRef"
           @scroll="handleContentScroll">
        <AddTodo @add-todo="addTodo" :current-theme="currentTheme"/>
        <TodoList 
          :todos="todos" 
          @toggle-todo="toggleTodo"
          @delete-todo="deleteTodo"
        />
      </div>
    </div>
  </div>
</template>

<script>
import AddTodo from './components/AddTodo.vue'
import TodoList from './components/TodoList.vue'

export default {
  name: 'App',
  components: {
    AddTodo,
    TodoList
  },
  data() {
    return {
      todos: [
        // {
        //   id: Date.now() - 4000,
        //   text: '完成项目文档',
        //   completed: false
        // },
        // {
        //   id: Date.now() - 3000,
        //   text: '学习Vue.js新特性',
        //   completed: true
        // },
        // {
        //   id: Date.now() - 2000,
        //   text: '准备周会演示',
        //   completed: false
        // },
        // {
        //   id: Date.now() - 1000,
        //   text: '回复重要邮件',
        //   completed: false
        // },
        //  {
        //   id: Date.now() - 500,
        //   text: '回复重aa要邮件',
        //   completed: false
        // },
        //  {
        //   id: Date.now() - 200,
        //   text: '回复重aa要邮件',
        //   completed: false
        // }
      ],
      scrollInfo: {
        scrollTop: 0,
        isScrollingUp: false,
        lastScrollTop: 0
      },
      scrollDebounceTimer: null, // 防抖定时器
      scrollTimeout: null, // 滚动停止防抖定时器
      showThemeMenu: false, // 主题菜单显示状态
      currentTheme:   {
          name: '深紫夜',
          primary: '#6366f1',
          secondary: '#8b5cf6',
          bgColors: [
            'rgba(120, 119, 198, 0.3)',
            'rgba(255, 119, 198, 0.3)',
            'rgba(120, 219, 255, 0.3)'
          ]
        }, // 当前主题
      themes: [
        {
          name: '深紫夜',
          primary: '#6366f1',
          secondary: '#8b5cf6',
          bgColors: [
            'rgba(120, 119, 198, 0.3)',
            'rgba(255, 119, 198, 0.3)',
            'rgba(120, 219, 255, 0.3)'
          ]
        },
        {
          name: '暗蓝海',
          primary: '#3b82f6',
          secondary: '#1d4ed8',
          bgColors: [
            'rgba(59, 130, 246, 0.3)',
            'rgba(34, 197, 94, 0.3)',
            'rgba(147, 197, 253, 0.3)'
          ]
        },
        {
          name: '墨绿林',
          primary: '#059669',
          secondary: '#047857',
          bgColors: [
            'rgba(5, 150, 105, 0.3)',
            'rgba(168, 85, 247, 0.3)',
            'rgba(34, 197, 94, 0.3)'
          ]
        },
        {
          name: '深青夜',
          primary: '#0891b2',
          secondary: '#0e7490',
          bgColors: [
            'rgba(8, 145, 178, 0.3)',
            'rgba(251, 113, 133, 0.3)',
            'rgba(165, 243, 252, 0.3)'
          ]
        },
        {
          name: '深金夜',
          primary: '#d97706',
          secondary: '#b45309',
          bgColors: [
            'rgba(217, 119, 6, 0.3)',
            'rgba(139, 92, 246, 0.3)',
            'rgba(254, 249, 195, 0.3)'
          ]
        }
      ]
    }
  },
  computed: {
    completedCount() {
      return this.todos.filter(todo => todo.completed).length
    },
    progressPercentage() {
      if (this.todos.length === 0) return 0
      return Math.round((this.completedCount / this.todos.length) * 100)
    },
    isOverflow(){
      return this.scrollInfo.scrollHeight > window.innerHeight
    }
  },
  methods: {
    addTodo(text) {
      this.todos.push({
        id: Date.now(),
        text,
        completed: false
      })
      // 确保新添加的待办事项在正确位置（虽然默认就在前面）
      this.sortTodos()
    },
    toggleTodo(id) {
      const todo = this.todos.find(t => t.id === id)
      if (todo) {
        todo.completed = !todo.completed
        // 重新排序：待办事项在前，已办事项在后
        this.sortTodos()
      }
    },
    sortTodos() {
      // 按照 completed 状态排序：false (待办) 在前，true (已办) 在后
      // 同状态内保持原有顺序（stable sort）
      this.todos.sort((a, b) => {
        if (a.completed === b.completed) {
          return 0 // 保持原有顺序
        }
        return a.completed ? 1 : -1 // 未完成的排在前面
      })
    },
    deleteTodo(id) {
      this.todos = this.todos.filter(t => t.id !== id)
    },
    handleContentScroll(event) {
      // 处理内容区域的滚动事件
      const target = event.target
      const currentScrollTop = target.scrollTop
      const scrollHeight = target.scrollHeight
      const pageHeight = window.innerHeight
      const isAtBottom = currentScrollTop + pageHeight >= scrollHeight - 10
      
      // 检测滚动方向
      const lastScrollTop = this.scrollInfo.lastScrollTop || 0
      const scrollDiff = currentScrollTop - lastScrollTop
      
      // 只有当内容高度超过页面高度时才启用头部收起功能
      const hasOverflow = scrollHeight > pageHeight
      
      let isScrollingUp = false

      
      // 使用if-else逻辑，避免状态变来变去
      if (hasOverflow) {
        // 有内容溢出的情况
        if (currentScrollTop === 0) {
          // 在顶部，始终显示完整头部
          isScrollingUp = false
        } else if (isAtBottom) {
          // 在底部，收起头部以节省空间
          isScrollingUp = true
        } else if (scrollDiff > 5) {
          // 向下滚动超过5px（scrollTop增加），收起头部
          isScrollingUp = true
        } else if (scrollDiff < -5) {
          // 向上滚动超过5px（scrollTop减少），显示头部
          isScrollingUp = false
        } else {
          // 滚动距离太小，保持当前状态
          isScrollingUp = this.scrollInfo.isScrollingUp
        }
      } else {
        // 没有内容溢出，始终显示完整头部
        isScrollingUp = false
      }
      
      // 特殊处理：如果在底部且需要收起头部，立即更新
      const isAtBottomAndShouldCollapse = isAtBottom && isScrollingUp && !this.scrollInfo.isScrollingUp

      // 只有当状态真正需要改变时才更新
      const shouldUpdate = 
        this.scrollInfo.isScrollingUp !== isScrollingUp ||
        Math.abs(this.scrollInfo.scrollTop - currentScrollTop) > 10 ||
        isAtBottomAndShouldCollapse
      
      if (shouldUpdate) {
        // 清除之前的防抖定时器
        if (this.scrollDebounceTimer) {
          clearTimeout(this.scrollDebounceTimer)
        }
        
        // 对于底部收起操作，立即更新；其他情况使用防抖
        const delay = isAtBottomAndShouldCollapse ? 0 : 30
        
        this.scrollDebounceTimer = setTimeout(() => {
          this.scrollInfo = {
            scrollTop: currentScrollTop,
            scrollHeight: scrollHeight,
            clientHeight: pageHeight,
            scrollPercentage: Math.round((currentScrollTop / (scrollHeight - pageHeight)) * 100),
            isScrollingUp: isScrollingUp,
            lastScrollTop: currentScrollTop,
            isAtBottom: isAtBottom
          }
        }, delay)
      } else {
        // 更新位置信息，但不改变显示状态
        this.scrollInfo.scrollTop = currentScrollTop
        this.scrollInfo.lastScrollTop = currentScrollTop
        this.scrollInfo.isAtBottom = isAtBottom
      }
      
      // 防抖处理
      if (this.scrollTimeout) {
        clearTimeout(this.scrollTimeout)
      }
    },
    toggleThemeMenu() {
      this.showThemeMenu = !this.showThemeMenu
    },
    selectTheme(theme) {
      this.currentTheme = theme
      // 更新CSS变量，只影响UI元素颜色，不改变背景
      document.documentElement.style.setProperty('--primary-color', theme.primary)
      document.documentElement.style.setProperty('--secondary-color', theme.secondary)
      // 更新背景颜色变量
      if (theme.bgColors && theme.bgColors.length >= 3) {
        document.documentElement.style.setProperty('--bg-color-1', theme.bgColors[0])
        document.documentElement.style.setProperty('--bg-color-2', theme.bgColors[1])
        document.documentElement.style.setProperty('--bg-color-3', theme.bgColors[2])
      }
      // 更新带透明度的颜色变量
      const primaryRgb = this.hexToRgb(theme.primary)
      const secondaryRgb = this.hexToRgb(theme.secondary)
      if (primaryRgb) {
        document.documentElement.style.setProperty('--primary-color-rgb', `${primaryRgb.r}, ${primaryRgb.g}, ${primaryRgb.b}`)
      }
      if (secondaryRgb) {
        document.documentElement.style.setProperty('--secondary-color-rgb', `${secondaryRgb.r}, ${secondaryRgb.g}, ${secondaryRgb.b}`)
      }
    },
    hexToRgb(hex) {
      const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
      return result ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16)
      } : null
    }
  },
  mounted() {
    // 初始化默认主题
    this.currentTheme = this.themes[0]
    this.selectTheme(this.currentTheme)
  },
  beforeUnmount() {
    // 清理定时器
    if (this.scrollTimeout) {
      clearTimeout(this.scrollTimeout)
    }
    if (this.scrollDebounceTimer) {
      clearTimeout(this.scrollDebounceTimer)
    }
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}


body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #0f0f23;
  min-height: 100vh;
  overflow: hidden;
  -webkit-app-region: drag; /* 允许拖拽窗口 */
}

#app {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  -webkit-app-region: no-drag; /* 内容区域不可拖拽 */
}

.container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #0f0f23;
  overflow: hidden;
  border-radius: 0; /* 移除圆角，因为窗口本身没有边框 */
  position: relative;

}


.bg {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 20% 50%, var(--bg-color-1, rgba(120, 119, 198, 0.3)) 0%, transparent 50%),
    radial-gradient(circle at 80% 20%, var(--bg-color-2, rgba(255, 119, 198, 0.3)) 0%, transparent 50%),
    radial-gradient(circle at 40% 80%, var(--bg-color-3, rgba(120, 219, 255, 0.3)) 0%, transparent 50%);
  z-index: 0;
}


.header {
  position: relative;
  padding: 50px 24px 40px;
  color: white;
  overflow: hidden;
  -webkit-app-region: drag; /* 头部可拖拽 */
  z-index: 1;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  min-height: 200px; /* 设置最小高度，确保头部高度稳定 */
  display: flex;
  flex-direction: column;
  justify-content: space-between; /* 改为space-between，让内容分布在两端 */
}

.header.compact {
  padding: 20px 24px 16px;
  transition: padding 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  min-height: 80px; /* 紧凑模式的最小高度 */
  justify-content: center; /* 紧凑模式下居中显示 */
}

/* 紧凑模式下的content高度 */
.header.compact + .content {
  height: calc(100vh - 80px); /* 紧凑模式下：视口高度减去紧凑头部高度 */
}

.header.compact .progress-section {
  margin-top: 20px;
}

.header.compact .progress-bar {
  height: 6px;
}

.header.compact .progress-text {
  font-size: 12px;
  margin-bottom: 0;
}

.header-content {
  position: relative;
  z-index: 1;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 30px;
  transition: opacity 0.3s ease;
}

.title-section h1 {
  font-size: 32px;
  font-weight: 800;
  letter-spacing: -1px;
  margin-bottom: 8px;
  background: linear-gradient(135deg, #fff 0%, var(--primary-color, #e0e7ff) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.subtitle {
  font-size: 16px;
  color: #a5b4fc;
  font-weight: 400;
}

.stats-section {
  display: flex;
  gap: 16px;
}

.stats-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 16px;
  padding: 16px 16px;
  text-align: center;
  width: 70px;
  height: 70px;
}

.stats-number {
  font-size: 24px;
  font-weight: 700;
  color: #fff;
  margin-bottom: 4px;
}

.stats-label {
  font-size: 12px;
  color: #a5b4fc;
  font-weight: 500;
}

/* 主题选择器样式 */
.theme-selector {
  position: fixed;
  top: 5px;
  right: 5px;
  z-index: 2000;
}

/* 调色盘切换按钮 */
.theme-toggle {
  width: 26px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-toggle img {
  opacity: 0.7;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-toggle:hover {
  transform: scale(1.1);
}

.theme-toggle:hover img {
  opacity: 0.9;
}

/* 主题菜单 */
.theme-menu {
  position: absolute;
  top: 0;
  right: 0;
  display: flex;
  align-items: center;
  gap: 3px;
  z-index: 1000;
  /* 初始状态：隐藏 */
  opacity: 0;
  visibility: hidden;
  transform: translateX(20px) scale(0.9);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-menu.expanded {
  /* 展开状态：显示 */
  opacity: 1;
  visibility: visible;
  transform: translateX(0) scale(1);
}

/* 收起按钮 */
.theme-collapse {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: rgba(255, 255, 255, 0.6);
  transition: all 0.3s ease;
}

.theme-collapse:hover {
  color: rgba(255, 255, 255, 0.9);
  transform: scale(1.1);
}

/* 主题选项 */
.theme-option {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  cursor: pointer;
  transition: all 0.3s ease;
  padding: 3px;
}

.theme-preview {
  width: 20px;
  height: 20px;
  border-radius: 6px;
  border: 2px solid rgba(255, 255, 255, 0.2);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-option:hover .theme-preview {
  border: 2px solid rgba(255, 255, 255, 0.4);
  transform: scale(1.1);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.15);
}

.theme-option.active .theme-preview {
  border: 2px solid rgba(255, 255, 255, 0.6);
  transform: scale(1.1);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
}



.progress-section {
  position: relative;
  z-index: 1;
}

.progress-bar {
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 12px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary-color, #8b5cf6) 0%, var(--secondary-color, #a855f7) 50%, var(--secondary-color, #d946ef) 100%);
  border-radius: 4px;
  transition: width 0.8s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 0 20px rgba(var(--primary-color-rgb, 139, 92, 246), 0.5);
}

.progress-text {
  font-size: 14px;
  color: #a5b4fc;
  font-weight: 500;
  text-align: center;
  margin-bottom: 0; /* 移除底部边距 */
}



.content {
  height: calc(100vh - 200px); /* 固定高度：视口高度减去头部高度 */
  padding: 24px;
  overflow-y: auto; /* 自动滚动 */
  overflow-x: hidden; /* 防止水平滚动 */
  display: flex;
  flex-direction: column;
  -webkit-app-region: no-drag; /* 内容区域不可拖拽 */
  position: relative;
  z-index: 1;
  min-height: 0; /* 确保flex子元素可以收缩 */
  box-sizing: border-box; /* 确保padding不会影响总宽度 */
  width: 100%; /* 确保占满父容器宽度 */
  /* 隐藏滚动条但保持滚动功能 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}



/* AddTodo和TodoList一起滚动 */
.content .add-todo {
  margin-bottom: 24px;
  width: 100%;
  flex-shrink: 0;
}

.content .todo-list {
  flex: 1;
  min-height: 0;
  width: 100%;
}

.content.no-scroll {
  overflow-y: hidden; /* 没有内容时隐藏滚动条 */
}

.content.no-scroll .todo-list {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}


/* 隐藏 WebKit 滚动条 */
.content::-webkit-scrollbar {
  display: none;
}

@keyframes bounce {
  0%, 20%, 50%, 80%, 100% {
    transform: translateY(0);
  }
  40% {
    transform: translateY(-6px);
  }
  60% {
    transform: translateY(-3px);
  }
}


</style>