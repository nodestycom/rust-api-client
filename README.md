# Nodesty Rust API Client

[![API](https://img.shields.io/badge/Nodesty-API-purple.svg)](https://nodesty.com)  
[![Author](https://img.shields.io/badge/Author-Staticius-blue.svg)](https://github.com/staticius)

**nodesty.com**'un güçlü API'sine Rust uygulamalarınızdan kolayca erişmenizi sağlayan modern, asenkron ve tip güvenli bir istemci kütüphanesi.

---

## 🚀 Özellikler

- ⚡ **Asenkron Operasyonlar**: Tam async destek!
- 🔒 **Tip Güvenliği**: Güçlü enum/struct yapılarıyla modelleme
- 🔄 **Otomatik Serileştirme**: JSON ↔ Rust veri tipleri dönüşümü `serde` ile
- ⚙️ **Yapılandırılabilir**: Timeout, retry, rate limit ayarları
- 🛡️ **Kapsamlı Hata Yönetimi**: Standart `ApiResponse<T>` yapısı
- 📦 **Modüler Yapı**: Her servis kendi modülü içinde

---

## 📋 Desteklenen Servisler

| Servis                | Açıklama                                                  | Erişim                      |
|-----------------------|-----------------------------------------------------------|-----------------------------|
| **User Service**      | Kullanıcı profili, hizmetler, faturalar, destek biletleri | `UserApiService`           |
| **VPS Service**       | VPS yönetimi, yedekler, şifre değişimi, istatistikler     | `VpsApiService`            |
| **Dedicated Service** | Dedicated sunucu yönetimi, donanım bilgileri              | `DedicatedServerApiService`|
| **Firewall Service**  | nShield kuralları, saldırı logları, rDNS yönetimi         | `FirewallApiService`       |

---

## 🛠️ Kurulum

Terminalinizde aşağıdaki komutu çalıştırın:

```shell
cargo add nodesty-api-library
```

---

## 🔑 Başlangıç

### API Token Alma

Nodesty dashboarddan bir erişim belirteci oluşturun.

### Temel Kullanım

```rust
use nodesty_api_library::{
    models::RestClientOptions,
    NodestyApiClient,
    services::{UserApiService, VpsApiService}
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("NODESTY_API_TOKEN").expect("Token yok");
    let options = RestClientOptions::new(token)
        .with_timeout_ms(45_000)
        .with_retry(5)
        .with_rate_limit_offset_ms(100);

    let client = Arc::new(NodestyApiClient::new(options)?);
    let user_service = UserApiService::new(client.clone());
    let vps_service = VpsApiService::new(client);

    let user_response = user_service.get_current_user().await?;
    if user_response.success {
        if let Some(user) = user_response.data {
            println!("Hoş geldin {}", user.full_name);
        } else {
            println!("Kullanıcı bilgisi bulunamadı.");
        }
    } else {
        println!("Kullanıcı bilgisi getirilirken hata oluştu: {:?}", user_response.error);
    }

    Ok(())
}
```

---

## 📖 Kullanım Örnekleri

### 👤 Kullanıcı Bilgileri

```rust
let user_service = UserApiService::new(client.clone());
let res = user_service.get_current_user().await?;
if res.success {
    let user = res.data.unwrap();
    println!("Merhaba {}", user.full_name);
    println!("Email: {}", user.email);
} else {
    println!("Hata: {:?}", res.error);
}
```

### 🖥️ VPS Yönetimi

```rust
let vps_service = VpsApiService::new(client.clone());
let vps_id = "your-vps-id";

use nodesty_api_library::models::vps::VpsAction;
vps_service.perform_action(vps_id, VpsAction::Restart).await?;

let backups = vps_service.get_backups(vps_id).await?.data.unwrap();
for backup in backups {
    println!("Yedek: {} - {}", backup.date, backup.file);
}
```

### 🔧 Dedicated Sunucu

```rust
let dedicated_service = DedicatedServerApiService::new(client.clone());
let id = "your-dedicated-id";
let res = dedicated_service.get_hardware_components(id).await?;
for part in res.data.unwrap() {
    println!("{}: {} {}{}", part.component, part.model, part.value, part.value_suffix);
}
```

### 🛡️ Güvenlik Duvarı

```rust
let firewall_service = FirewallApiService::new(client.clone());
use nodesty_api_library::models::firewall::FirewallCreateRuleData;

let rule = FirewallCreateRuleData { port: 25565, app_id: 123 };
firewall_service.create_rule("svc-id", "ip", rule).await?;

let logs = firewall_service.get_attack_logs("svc-id", "ip").await?.data.unwrap();
for log in logs {
    println!("Saldırı Başlangıcı: {} - Vektörler: {:?}", log.started_at, log.vectors);
}
```

---

## 🏗️ API Yanıt Yapısı

Tüm API çağrıları `ApiResponse<T>` döner:

```rust
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}
```

---

## ⚙️ Yapılandırma Seçenekleri

```rust
let options = RestClientOptions::new(token)
    .with_timeout_ms(30_000)
    .with_retry(3)
    .with_rate_limit_offset_ms(50);
```

---

## 📚 API Servisleri

### 👤 User Service (`UserApiService`)

| Metod                 | Açıklama                          | Endpoint                           |
|-----------------------|-----------------------------------|-------------------------------------|
| `get_current_user()`  | Kullanıcı profilini al            | `GET /users/@me`                    |
| `get_services()`      | Hizmetleri listele                | `GET /services`                     |
| `get_tickets()`       | Destek biletlerini listele        | `GET /tickets`                      |
| `get_ticket(id)`      | Belirli bileti getir              | `GET /tickets/{id}`                 |
| `get_invoices()`      | Faturaları listele                | `GET /users/@me/invoices`           |
| `get_invoice(id)`     | Belirli faturayı getir            | `GET /users/@me/invoices/{id}`      |
| `get_sessions()`      | Aktif oturumları getir            | `GET /users/@me/sessions`           |

### 🖥️ VPS Service (`VpsApiService`)

| Metod                 | Açıklama                          | Endpoint                             |
|-----------------------|-----------------------------------|---------------------------------------|
| `perform_action()`    | VPS eylemi gerçekleştir           | `POST /services/{id}/vps/action`     |
| `get_backups()`       | Yedekleri getir                   | `GET /services/{id}/vps/backups`     |
| `restore_backup()`    | Geri yükleme yap                  | `POST /services/{id}/vps/backups/...`|
| `change_password()`   | Şifre değiştir                    | `POST /services/{id}/vps/change-password` |
| `get_usage_statistics()` | İstatistikleri al              | `GET /services/{id}/vps/graphs`      |
| `get_details()`       | VPS detayları                     | `GET /services/{id}/vps/info`        |
| `get_os_templates()`  | OS şablonlarını getir             | `GET /services/{id}/vps/os-templates`|
| `reinstall()`         | Yeniden kurulum                   | `POST /services/{id}/vps/reinstall`  |
| `get_tasks()`         | Görevleri getir                   | `GET /services/{id}/vps/tasks`       |

### 🔧 Dedicated Service (`DedicatedServerApiService`)

| Metod                 | Açıklama                          | Endpoint                             |
|-----------------------|-----------------------------------|---------------------------------------|
| `perform_action()`    | Eylem çalıştır                    | `POST /services/{id}/dedicated/action`|
| `get_details()`       | Sunucu bilgisi                    | `GET /services/{id}/dedicated/info`   |
| `get_hardware_components()` | Donanım detayları          | `GET /services/{id}/dedicated/hardware`|
| `get_os_templates()`  | OS şablonları                     | `GET /services/{id}/dedicated/os-templates`|
| `get_reinstall_status()` | Kurulum durumu                 | `GET /services/{id}/dedicated/reinstall-status`|
| `reinstall()`         | Yeniden kur                       | `POST /services/{id}/dedicated/reinstall`|
| `get_tasks()`         | Görevleri getir                   | `GET /services/{id}/dedicated/tasks`  |

### 🛡️ Firewall Service (`FirewallApiService`)

| Metod                             | Açıklama                          | Endpoint                                  |
|-----------------------------------|-----------------------------------|--------------------------------------------|
| `get_attack_logs()`               | Saldırı kayıtlarını getir         | `GET /services/{id}/firewall/{ip}/attack-logs` |
| `get_attack_notification_settings()` | Bildirim ayarları              | `GET /services/{id}/firewall/{ip}/attack-notification` |
| `update_attack_notification_settings()` | Bildirim ayarlarını güncelle | `PUT /services/{id}/firewall/{ip}/attack-notification` |
| `reset_reverse_dns()`             | rDNS kaldır                       | `DELETE /services/{id}/firewall/{ip}/rdns` |
| `get_reverse_dns()`               | rDNS bilgisi                      | `GET /services/{id}/firewall/{ip}/rdns` |
| `upsert_reverse_dns()`            | rDNS ayarla                       | `PUT /services/{id}/firewall/{ip}/rdns` |
| `delete_rule()`                   | Kural sil                         | `DELETE /services/{id}/firewall/{ip}/rules/{rule_id}` |
| `get_rules()`                     | Kuralları getir                   | `GET /services/{id}/firewall/{ip}/rules` |
| `create_rule()`                   | Kural oluştur                     | `POST /services/{id}/firewall/{ip}/rules` |
| `get_statistics()`                | İstatistikleri getir              | `GET /services/{id}/firewall/{ip}/stats` |

---

## 🔐 Güvenlik İpuçları

- Token'ı `.env` dosyasından veya ortam değişkeninden alın
- Her `ApiResponse` sonrası `.success` kontrolü yapın
- Hataları `match` veya `if let` ile detaylı yönetin

---

## 🚀 Performans Önerileri

- `tokio::join!` ile işlemleri paralel çalıştırın
- Rate limit için `rate_limit_offset_ms` yapılandırmasını ayarlayın
- Uygulama başında istemciyi bir kez oluşturun ve tekrar kullanın

---

## 🧪 Test Örneği

```rust
#[tokio::test]
async fn test_user_service() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("NODESTY_TEST_TOKEN").unwrap();
    let client = Arc::new(NodestyApiClient::new(RestClientOptions::new(token))?);
    let user_service = UserApiService::new(client);

    let response = user_service.get_current_user().await?;
    assert!(response.success);
    assert!(response.data.unwrap().email.contains("@"));

    Ok(())
}
```

---

## 🐛 Sorun Giderme

- **401 Unauthorized**: Token geçersiz veya süresi dolmuş olabilir
- **Timeout**: Ağ bağlantısını ve timeout süresini kontrol edin
- **Rate Limiting**: `rate_limit_offset_ms` değerini artırın veya daha az istek gönderin

---

## 🤝 Katkıda Bulunma

1. Repo'yu forklayın
2. Yeni bir branch oluşturun (`git checkout -b feature/harika-ozellik`)
3. Commit ve push yapın
4. PR gönderin

---

## 🔗 Bağlantılar

- 🌐 [Nodesty Website](https://nodesty.com)
- 📘 [API Dökümantasyonu](https://nodesty.com/api)

---

## ⭐ Destek

Bu proje faydalı olduysa ⭐ vererek destekleyebilirsiniz!

**Made with ❤️ for Nodesty Community.**