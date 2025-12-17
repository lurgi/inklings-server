# 코딩 스타일

## 주석 규칙

### 기본 원칙
- **불필요한 주석은 작성하지 않는다**
- 코드 자체로 의미가 명확하면 주석 불필요
- 주석이 필요한 경우:
  - 복잡한 비즈니스 로직
  - 왜 이렇게 구현했는지 (Why)
  - 외부 API나 복잡한 알고리즘
- 주석이 불필요한 경우:
  - 코드가 하는 일을 그대로 반복 (What)
  - 함수명/변수명으로 충분히 설명 가능한 경우

### 나쁜 예
```rust
// 사용자 ID를 가져옴
let user_id = get_user_id();

// 사용자를 생성함
let user = create_user();
```

### 좋은 예
```rust
let user_id = get_user_id();
let user = create_user();

// 외부 결제 API는 3번까지 재시도 필요 (API 문서 참고)
let payment_result = retry_payment_api(3).await?;
```

---

## Rust 코딩 표준

### 네이밍 컨벤션
- 함수명: `snake_case`
- 상수명: `SCREAMING_SNAKE_CASE`
- 타입명: `PascalCase`

### 에러 처리
- `unwrap()` 사용 금지 - `?` 또는 `Result` 타입 사용
- 프로덕션 코드에서 `expect()` 사용 금지

### 문서화
- 모든 public 함수/struct는 `///` 문서화 주석 작성

### Async/Await 규칙

#### 비동기 함수 정의
**설명**: DB나 외부 API 호출은 모두 `async fn`으로 작성한다.

**좋은 예시**:
```rust
impl UserService {
    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, ServiceError> {
        let existing = self.user_repo.find_by_email(&req.email).await?;
        // ...
    }
}
```

**나쁜 예시**:
```rust
// 동기 함수에서 비동기 호출
pub fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, ServiceError> {
    let existing = self.user_repo.find_by_email(&req.email); // await 불가능
}
```

**이유**: Tokio 런타임에서는 모든 I/O 작업이 비동기여야 한다.

#### `?` 연산자 활용
**설명**: `Result`를 반환하는 비동기 함수에서는 `?`로 에러 전파.

**좋은 예시**:
```rust
pub async fn get_user_surveys(&self, user_id: i32) -> Result<Vec<SurveyResponse>, ServiceError> {
    let user = self.user_repo.find_by_id(user_id).await?;
    let surveys = self.survey_repo.find_by_user_id(user_id).await?;
    Ok(surveys.into_iter().map(Into::into).collect())
}
```

**나쁜 예시**:
```rust
pub async fn get_user_surveys(&self, user_id: i32) -> Result<Vec<SurveyResponse>, ServiceError> {
    match self.user_repo.find_by_id(user_id).await {
        Ok(user) => {
            match self.survey_repo.find_by_user_id(user_id).await {
                Ok(surveys) => Ok(surveys.into_iter().map(Into::into).collect()),
                Err(e) => Err(e.into()),
            }
        }
        Err(e) => Err(e.into()),
    }
}
```

**이유**: `?` 연산자는 코드를 간결하게 만들고 가독성을 높인다.
