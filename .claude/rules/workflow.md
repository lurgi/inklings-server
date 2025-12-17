# 기능 추가 워크플로우

새로운 기능을 추가할 때는 다음 단계를 따른다.

## 1. 요구사항 분석 단계
- **API 스펙 정의**: 엔드포인트, HTTP 메서드, 요청/응답 형식
- **필요한 데이터 파악**: 어떤 데이터를 저장하고 조회할지
- **비즈니스 로직 파악**: 유효성 검증, 권한 체크, 중복 확인 등

## 2. 설계 단계
- **DB 스키마 설계**: Entity 필드, 타입, 제약조건, 관계 정의
- **DTO 설계**: Request DTO (검증 규칙 포함), Response DTO (민감 정보 제외)
- **에러 케이스 정의**: 발생 가능한 에러와 HTTP 상태 코드

## 3. 구현 단계 (Bottom-up)

**순서를 반드시 지킨다:**

### 1. Entity + Migration
- `src/entities/` 에 Entity 정의
- `migration/src/` 에 마이그레이션 작성
- `sea-orm-cli migrate up` 실행하여 DB 스키마 생성

### 2. Repository
- `src/repositories/` 에 Repository 구현
- CRUD 메서드 작성 (find_*, create, update, delete)
- 복잡한 쿼리는 테스트 작성

### 3. Service
- `src/services/` 에 Service 구현
- Repository를 조합하여 비즈니스 로직 작성
- **필수: 단위 테스트 작성**

### 4. Handler
- `src/handlers/` 에 Handler 구현
- DTO 검증, Service 호출, 응답 반환
- 라우터에 엔드포인트 등록

## 4. 테스트 및 검증
- Service 테스트 실행 (`cargo test`)
- 필요시 통합 테스트 작성
- API 수동 테스트 (curl, Postman 등)
