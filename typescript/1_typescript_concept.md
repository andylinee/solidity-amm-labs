## 重要概念說明
### Node.js vs Browser JavaScript
* Node.js 讓 JavaScript 可以在電腦上執行（不只是瀏覽器）
* 我們用它來建立 CLI 工具、伺服器等

### TypeScript vs JavaScript
* TypeScript = JavaScript + 型別系統
* 編譯時會檢查錯誤，執行時轉換成 JavaScript

### 套件說明
* `commander`: 處理 CLI 指令（類似 Python 的 argparse）
* `chalk`: 終端機文字顏色
* `inquirer`: 互動式命令列介面
* `ts-node`: 直接執行 TypeScript（開發用）

---

## TypeScript 概念解析
### 1. Enum (列舉)

```code=
typescriptexport enum TaskStatus {
  TODO = 'todo',
  IN_PROGRESS = 'in_progress', 
  COMPLETED = 'completed'
}
```

* 類似 Python 的 `Enum` 或 Rust 的 `enum`
* 提供一組預定義的常數值
* 比字串更安全，有自動完成功能

### 2. Interface (介面)
```code=
typescriptexport interface Task {
  id: string;
  title: string;
  description?: string;  // ? 表示可選屬性
}
```

* 定義物件的結構和型別
* 類似 Python 的 `TypedDict` 或 Rust 的 `struct`
* `?` 代表可選屬性

### 3. 泛型 (Generics)
```code=
typescriptexport interface ApiResponse<T> {
  data?: T;  // T 是型別參數
}
```

* 讓型別可以重複使用
* 類似 Python 的 `Generic[T]` 或 Rust 的 `<T>`

### 4. 型別守衛 (Type Guards)
```code=
typescriptexport function isValidTaskStatus(status: string): status is TaskStatus {
  return Object.values(TaskStatus).includes(status as TaskStatus);
}
```

* `status is TaskStatus` 是型別斷言
* 幫助 TypeScript 推斷型別

--- 

## TypeScript 語言概念
### 1. 非同步程式設計
```code=
typescriptexport async function loadTasks(): Promise<Task[]> {
  // async/await 類似 Python 的協程
  const data = await fs.promises.readFile(TASKS_FILE, 'utf8');
}
```

### 2. 錯誤處理
```code=
typescripttry {
  // 危險操作
} catch (error) {
  console.error('發生錯誤:', error);
  return [];
}
```

### 3. 模組匯入/匯出
```code=
typescriptimport * as fs from 'fs';           // 匯入整個模組
import { Task } from '../types/Task'; // 匯入特定型別
export async function loadTasks()    // 匯出函數
```

### 4. 型別註釋
```code=
typescriptfunction saveTasks(tasks: Task[]): Promise<boolean>
//               參數型別    回傳值型別
```

---

## TypeScript 語言進階概念
### 1. 類別 (Classes)
```code=
typescriptexport class TaskService {
  private tasks: Task[] = [];        // 私有屬性
  private isLoaded: boolean = false; // 型別推斷
  
  async initialize(): Promise<void>  // 公開方法
}
```

### 2. 存取修飾符

* `private`: 只能在類別內部存取
* `public`: 預設，可以從外部存取
* 類似 Python 的 `_` 約定但更嚴格

### 3. 展開運算子 (Spread Operator)
```code=
typescriptconst updatedTask: Task = {
  ...currentTask,    // 複製所有屬性
  ...updateData,     // 覆寫指定屬性
  updatedAt: new Date()
};
```

### 4. 可選串聯 (Optional Chaining)
```code=
typescripttitle: updateData.title?.trim() ?? currentTask.title
//                     ?         ?? (nullish coalescing)
```

### 5. 型別守衛與錯誤處理
```code=
typescripterror instanceof Error ? error.message : '未知錯誤'
```
