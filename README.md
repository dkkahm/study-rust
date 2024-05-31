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
    - scope가 지정된 mock
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
    - context (or with_context) 를 사용하거나 anyhow::Error를 리턴하거나
- 상위로 전파되는 로그는 로깅할 필요없음

### 인증
- Argon2
- 

```
[dependencies]
aes = "0.7"
base64 = "0.22.1"
block-modes = "0.8"
block-padding = "0.2"
```

```
fn encrypt_message(message: &str, password: &str) -> Result<String, anyhow::Error> {
    let password = password.as_bytes();

    // Generate a random IV
    let mut iv = [0u8; IV_SIZE];
    rand::thread_rng().fill(&mut iv);
    
    // Create the cipher instance
    let cipher = Aes256Cbc::new_var(password, &iv)?;
    
    // Encrypt the message
    let ciphertext = cipher.encrypt_vec(message.as_bytes());
    
    Ok(format!("{:x}", iv.to_vec().extend(ciphertext)))
}
```

위의 코드에서 Aes256Cbc::new_var를 포함한 라인과 마지막 Ok(format!("{:x}", iv.to_vec().extend(ciphertext))) 라인에서 발생하는 오류룰 수정해주세요.