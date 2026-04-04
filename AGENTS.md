# AGENTS.md - Development Guidelines

## Project Overview

This is a full-stack application with:
- **Frontend**: Vue 3 + Vite + Pinia + Vuetify 3 + Tailwind CSS 4 (JavaScript, no TypeScript)
- **Backend**: Rust with Actix-web + Diesel ORM + PostgreSQL

## Build Commands

### Frontend (`reestr_front/`)
```bash
# Install dependencies
npm install

# Development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Run unit tests
npm run test:unit

# Run a single test file
npm run test:unit -- path/to/test.spec.js

# Run tests in watch mode
npm run test:unit -- --watch

# Format code with Prettier
npm run format
```

### Backend (`reestr_back/`)
```bash
# Install Rust dependencies
cargo build

# Run the backend server
cargo run

# Run tests
cargo test

# Run a single test
cargo test test_name

# Check code (linting)
cargo check

# Format code
cargo fmt
```

## Code Style Guidelines

### Frontend (Vue 3 + JavaScript)

#### Imports
- Use `@` alias for imports from `src/` directory
- Order: Vue core → Vue router/pinia → Third-party → Internal components/stores → Assets
- Example:
```javascript
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/store/auth'
import SomeComponent from '@/components/SomeComponent.vue'
import '@/assets/styles.css'
```

#### Formatting (Prettier)
- **Single quotes** for strings (no semicolons)
- **Print width**: 100 characters
- Configuration in `.prettierrc.json`:
```json
{
  "semi": false,
  "singleQuote": true,
  "printWidth": 100
}
```

#### Vue Component Style
- Use `<script setup>` for new components when possible
- Use Composition API (`ref`, `computed`, `reactive`)
- Component names: PascalCase (e.g., `ResponsiblePersonList.vue`)
- Use kebab-case for HTML attributes and props

#### State Management (Pinia)
- Use `defineStore` with arrow function syntax
- State: Use `state: () => ({ ... })`
- Actions: Async functions for API calls
- Getters: Use `this` or first argument

#### API/Axios
- Use the centralized `apiClient` from `@/axios.js`
- Handle errors with try/catch blocks
- Display errors using the toast system or `alert()`

#### Naming Conventions
- **Components**: PascalCase (`UserForm.vue`)
- **Variables/functions**: camelCase
- **Constants**: UPPER_SNAKE_CASE
- **CSS classes**: kebab-case (Tailwind)

### Backend (Rust)

#### Error Handling
- Services return `Result<T, String>` with descriptive error messages
- Handlers use `response_fn()` helper to convert results to HTTP responses
- Example pattern:
```rust
pub async fn some_action(data: SomeDTO) -> Result<SomeModel, String> {
    // ... operation ...
    .map_err(|e| format!("Error to do thing: {}", e))
}
```

#### Database (Diesel)
- Use Diesel macros: `#[derive(Insertable, Queryable, Serialize, Deserialize)]`
- Follow schema conventions from `schema.rs`
- Use `.filter()`, `.order()`, `.limit()` for queries

#### Module Structure
```
src/
├── main.rs           # Entry point, App configuration
├── handlers/         # HTTP handlers (route logic)
├── services/         # Business logic, DB operations
├── models/           # Data models, DTOs
├── schema.rs         # Diesel schema
├── auth/             # Authentication, JWT, roles
├── middleware/       # Custom middleware
├── conf/             # Configuration
└── utils/            # Helpers (DB, validation, etc.)
```

#### Naming (Rust)
- **Functions/variables**: snake_case
- **Types/Structs**: PascalCase
- **Modules**: snake_case
- **Constants**: SCREAMING_SNAKE_CASE

### General Guidelines

1. **Authentication**: JWT tokens stored in localStorage, sent via `Authorization: Bearer` header
2. **API Routes**: All prefixed with `/api/v1/`
3. **Roles**: `'admin'`, `'manager'`, (checked via `authStore.user.role`)
4. **Database**: PostgreSQL with Diesel ORM, migrations in `migrations/`
5. **Logging**: Backend uses `simple_logger`, logs to `application.log`
6. **Environment**: Variables in `.env` files (never commit secrets). Backend requires `JWT_SECRET` (≥32 characters) for JWT and session cookies.

### Testing

- Frontend tests: Vitest with jsdom environment
- Test files: `*.spec.js` or `*.test.js` in same directory as source
- Run single test: `npm run test:unit -- --run path/to/file.spec.js`
- Backend tests: Rust built-in test framework with `#[cfg(test)]`

### Database Migrations

```bash
# Create new migration
diesel migration generate migration_name

# Run migrations
diesel migration run

# Revert last migration
diesel migration revert
```
