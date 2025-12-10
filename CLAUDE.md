# Inklings Server - 프로젝트 규칙

## 코딩 스타일

### 주석 규칙
- **불필요한 주석은 작성하지 않는다**
- 코드 자체로 의미가 명확하면 주석 불필요
- 주석이 필요한 경우:
  - 복잡한 비즈니스 로직
  - 왜 이렇게 구현했는지 (Why)
  - 외부 API나 복잡한 알고리즘
- 주석이 불필요한 경우:
  - 코드가 하는 일을 그대로 반복 (What)
  - 함수명/변수명으로 충분히 설명 가능한 경우

**나쁜 예:**
```rust
// 사용자 ID를 가져옴
let user_id = get_user_id();

// 사용자를 생성함
let user = create_user();
```

**좋은 예:**
```rust
let user_id = get_user_id();
let user = create_user();

// 외부 결제 API는 3번까지 재시도 필요 (API 문서 참고)
let payment_result = retry_payment_api(3).await?;
```

### Rust 코딩 표준
- 함수명: `snake_case`
- 상수명: `SCREAMING_SNAKE_CASE`
- 타입명: `PascalCase`
- `unwrap()` 사용 금지 - `?` 또는 `Result` 타입 사용
- 모든 public 함수/struct는 `///` 문서화 주석 작성

## 프로젝트 구조

### 기술 스택
- **언어:** Rust
- **데이터베이스:** PostgreSQL
- **ORM:** SeaORM (SQL 작성 불필요)
- **Async Runtime:** Tokio

### 디렉토리 구조
```
inklings-server/
├── src/
│   ├── main.rs           # 엔트리 포인트
│   ├── db/              # 데이터베이스 연결
│   └── entities/        # SeaORM Entity 모델
└── migration/           # 데이터베이스 마이그레이션 (Rust 코드)
```

## SeaORM 사용 규칙

### Entity 정의
- `src/entities/` 에 모델 정의
- SQL 작성 불필요, Rust 코드로 정의
- `#[derive(DeriveEntityModel)]` 사용

### 마이그레이션
- `migration/src/` 에 Rust 코드로 작성
- SQL 파일 작성 금지
- 새 마이그레이션은 `m<timestamp>_<name>.rs` 형식

### CRUD 작업
- SeaORM의 ActiveModel 패턴 사용
- Raw SQL 쿼리 지양
- 타입 안전성 최대한 활용

## 에러 처리
- `unwrap()`, `expect()` 사용 금지 (프로덕션 코드)
- `Result` 타입과 `?` 연산자 사용
- `anyhow::Result` 활용

## 환경 변수
- `.env` 파일 사용 (git 무시)
- `.env.example` 템플릿 유지
- 민감 정보 절대 하드코딩 금지

## 개발 워크플로우
1. 기능 구현 전 Entity 정의
2. 마이그레이션 작성 및 실행
3. 비즈니스 로직 구현
4. 테스트 작성
