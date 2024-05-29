### 입출력
- 입력
    - Path
    - Query
    - Body
    - ...
    - 검증
        - 타입주도개발
    - 살균
- 출력
    - impl Responder
        - HttpResponse

### Test
- mock
- fake

### 설정 (configuration)
- 기본
- 계층적 구성
- 환경변수

### 상태

### Database
- transaction

### Logging
- log crate
- tracing
- secrecy

### 오류핸들링
- 불투명한 오류
- thiserror, anyhow
- 상위로 전파되는 로그는 로깅할 필요없음