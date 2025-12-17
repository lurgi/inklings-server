# Inklings Server - 프로젝트 규칙

## 📁 규칙 파일 구조

이 프로젝트는 다음 규칙 파일들을 따릅니다. 상세한 내용은 각 파일을 참조하세요.

- **[코딩 스타일](./.claude/rules/coding-style.md)**: 주석 규칙, Rust 표준, Async/Await
- **[아키텍처](./.claude/rules/architecture.md)**: 3계층 구조, SeaORM, 에러 처리, 계층별 상세 규칙
- **[워크플로우](./.claude/rules/workflow.md)**: 기능 추가 단계별 프로세스
- **[테스트](./.claude/rules/testing.md)**: Service/Repository/Handler 테스트 기준

---

## Claude Code 작업 규칙

### Git 작업 규칙
- **절대 사용자 승인 없이 `git push`를 실행하지 않는다**
- 커밋은 사용자가 명시적으로 요청했을 때만 수행
- Push 전에 반드시 사용자에게 변경 사항을 확인받는다

### 사고 과정 (Thinking Process)
- **복잡한 문제 해결 시 Sequential Thinking MCP를 사용하여 단계적으로 사고한다**
- **코드베이스 분석 시 Context7 MCP를 활용하여 맥락을 파악한다**
- 문제를 작은 단위로 나누어 접근한다
- 가정을 명확히 하고, 불확실한 부분은 사용자에게 질문한다

---

## 빠른 참조

### 기술 스택
- **언어:** Rust
- **웹 프레임워크:** Axum
- **데이터베이스:** PostgreSQL
- **ORM:** SeaORM
- **Async Runtime:** Tokio

### 3계층 아키텍처
```
Handler (HTTP) → Service (비즈니스 로직) → Repository (DB 접근)
```

### 구현 순서 (Bottom-up)
```
1. Entity + Migration
2. Repository
3. Service (+ 필수 테스트)
4. Handler
```

### 핵심 원칙
- ❌ `unwrap()` 사용 금지 → ✅ `?` 연산자 사용
- ❌ Entity 직접 노출 금지 → ✅ DTO 사용
- ❌ Handler에서 DB 직접 접근 금지 → ✅ Service 호출
- ❌ 불필요한 주석 금지 → ✅ 코드로 의미 표현

---

**자세한 규칙은 [`.claude/rules/`](./.claude/rules/) 폴더를 참조하세요.**
