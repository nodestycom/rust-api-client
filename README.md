# Nodesty Rust API Client

[![API](https://img.shields.io/badge/Nodesty-API-green.svg)](https://nodesty.com)
[![Author](https://img.shields.io/badge/Author-Staticius-blue.svg)](https://github.com/staticius)

&#x20;&#x20;

nodesty.com'un güçlü API'sine Rust uygulamalarınızdan kolayca erişmenizi sağlayan modern, asenkron ve tip güvenli bir
istemci kütüphanesi.

## 🚀 Özellikler

- **⚡ Asenkron Operasyonlar**: `tokio` + `reqwest` ile tam async destek
- **🔒 Tip Güvenliği**: Güçlü enum/struct yapılarıyla modelleme
- **🔄 Otomatik Serileştirme**: JSON ↔ Rust veri tipleri dönüşümü `serde` ile
- **⚙️ Yapılandırılabilir**: Timeout, retry, rate limit ayarları
- **🛡️ Kapsamlı Hata Yönetimi**: Standart `ApiResponse<T>` yapısı
- **📦 Modüler Yapı**: Her servis kendi modülü içinde

## 📋 Desteklenen Servisler

| Servis                | Açıklama                                                  | Erişim                    |
|-----------------------|-----------------------------------------------------------|---------------------------|
| **User Service**      | Kullanıcı profili, hizmetler, faturalar, destek biletleri | `client.user`             |
| **VPS Service**       | VPS yönetimi, yedekler, şifre değişimi, istatistikler     | `client.vps`              |
| **Dedicated Service** | Dedicated sunucu yönetimi, donanım bilgileri              | `client.dedicated_server` |
| **Firewall Service**  | nShield kuralları, saldırı logları, rDNS yönetimi         | `client.firewall`         |

## 🛠️ Kurulum

https://crates.io/crates/nodesty-api-library

## 🔑 Başlangıç

### API Token Alma

1. [Nodesty kontrol paneli](https://nodesty.com/dashboard/my-account/access-tokens) adresine giriş yapın.

### Temel Kullanım

```rust
use nodesty_client::{NodestyApiClient, RestClientOptions};

#[tokio::main]
async fn main() {
    let token = std::env::var("NODESTY_API_TOKEN").expect("Token yok");
    let options = RestClientOptions::new(token)
        .with_timeout(Duration::from_secs(45))
        .with_retry(5)
        .with_rate_limit_offset(100);

    let client = NodestyApiClient::new(options);

    let user = client.user.get_current_user().await.unwrap();
    println!("Hos geldin {}", user.data.unwrap().name);
}
```

## 📖 Kullanım Örnekleri

### 👤 Kullanıcı Bilgileri

```rust
let res = client.user.get_current_user().await?;
if res.success {
let user = res.data.unwrap();
println ! ("Merhaba {}", user.full_name);
println ! ("Email: {}", user.email);
} else {
println ! ("Hata: {:?}", res.error);
}
```

### 🖥️ VPS Yönetimi

```rust
let vps_id = "your-vps-id";

use nodesty_client::models::VpsAction;
client.vps.perform_action(vps_id, VpsAction::Reboot).await?;

let backups = client.vps.get_backups(vps_id).await?.data.unwrap();
for backup in backups {
println!("Yedek: {} - {}", backup.date, backup.file);
}
```

### 🔧 Dedicated Sunucu

```rust
let id = "your-dedicated-id";
let res = client.dedicated_server.get_hardware_components(id).await?;
for part in res.data.unwrap() {
println ! ("{}: {} {}{}", part.component, part.model, part.value, part.value_suffix);
}
```

### 🛡️ Güvenlik Duvarı

```rust
use nodesty_client::models::FirewallCreateRuleData;

let rule = FirewallCreateRuleData { port: 25565, appId: 123 };
client.firewall.create_rule("svc-id", "ip", rule).await?;

let logs = client.firewall.get_attack_logs("svc-id", "ip").await?.data.unwrap();
for log in logs {
println!("Saldırı: {} - {}", log.timestamp, log.attack_type);
}
```

## 🏗️ API Yanıt Yapısı

Tüm API çağrıları `ApiResponse<T>` döner:

```rust
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}
```

## ⚙️ Yapılandırma Seçenekleri

```rust
let options = RestClientOptions::new(token)
.with_timeout(Duration::from_secs(30))
.with_retry(3)
.with_rate_limit_offset(50);
```

## 📚 API Servisleri

### 👤 User Service (`client.user`)

| Metod                | Açıklama                   | Endpoint                       |
|----------------------|----------------------------|--------------------------------|
| `get_current_user()` | Kullanıcı profilini al     | `GET /users/@me`               |
| `get_services()`     | Hizmetleri listele         | `GET /services`                |
| `get_tickets()`      | Destek biletlerini listele | `GET /tickets`                 |
| `get_ticket(id)`     | Belirli bileti getir       | `GET /tickets/{id}`            |
| `get_invoices()`     | Faturaları listele         | `GET /users/@me/invoices`      |
| `get_invoice(id)`    | Belirli faturayı getir     | `GET /users/@me/invoices/{id}` |
| `get_sessions()`     | Aktif oturumları getir     | `GET /users/@me/sessions`      |

### 🖥️ VPS Service (`client.vps`)

| Metod                            | Açıklama                | Endpoint                                        |
|----------------------------------|-------------------------|-------------------------------------------------|
| `perform_action(id, action)`     | VPS eylemi gerçekleştir | `POST /services/{id}/vps/action`                |
| `get_backups(id)`                | Yedekleri getir         | `GET /services/{id}/vps/backups`                |
| `restore_backup(id, date, file)` | Geri yükleme yap        | `POST /services/{id}/vps/backups/{date}/{file}` |
| `change_password(id, data)`      | Şifre değiştir          | `POST /services/{id}/vps/change-password`       |
| `get_graphs(id)`                 | İstatistikleri al       | `GET /services/{id}/vps/graphs`                 |
| `get_details(id)`                | VPS detayları           | `GET /services/{id}/vps/info`                   |
| `get_os_templates(id)`           | OS şablonlarını getir   | `GET /services/{id}/vps/os-templates`           |
| `reinstall(id, data)`            | Yeniden kurulum         | `POST /services/{id}/vps/reinstall`             |
| `get_tasks(id)`                  | Görevleri getir         | `GET /services/{id}/vps/tasks`                  |

### 🔧 Dedicated Service (`client.dedicated_server`)

| Metod                         | Açıklama          | Endpoint                                        |
|-------------------------------|-------------------|-------------------------------------------------|
| `perform_action(id, action)`  | Eylem çalıştır    | `POST /services/{id}/dedicated/action`          |
| `get_hardware_components(id)` | Donanım detayları | `GET /services/{id}/dedicated/hardware`         |
| `get_details(id)`             | Sunucu bilgisi    | `GET /services/{id}/dedicated/info`             |
| `get_os_templates(id)`        | OS şablonları     | `GET /services/{id}/dedicated/os-templates`     |
| `get_reinstall_status(id)`    | Kurulum durumu    | `GET /services/{id}/dedicated/reinstall-status` |
| `reinstall(id, data)`         | Yeniden kur       | `POST /services/{id}/dedicated/reinstall`       |
| `get_tasks(id)`               | Görevleri getir   | `GET /services/{id}/dedicated/tasks`            |

### 🛡️ Firewall Service (`client.firewall`)

| Metod                                               | Açıklama                     | Endpoint                                               |
|-----------------------------------------------------|------------------------------|--------------------------------------------------------|
| `get_attack_logs(id, ip)`                           | Saldırı kayıtlarını getir    | `GET /services/{id}/firewall/{ip}/attack-logs`         |
| `get_attack_notification_settings(id, ip)`          | Bildirim ayarları            | `GET /services/{id}/firewall/{ip}/attack-notification` |
| `update_attack_notification_settings(id, ip, data)` | Bildirim ayarlarını güncelle | `PUT /services/{id}/firewall/{ip}/attack-notification` |
| `delete_reverse_dns(id, ip)`                        | rDNS kaldır                  | `DELETE /services/{id}/firewall/{ip}/rdns`             |
| `get_reverse_dns(id, ip)`                           | rDNS bilgisi                 | `GET /services/{id}/firewall/{ip}/rdns`                |
| `upsert_reverse_dns(id, ip, rdns)`                  | rDNS ayarla                  | `PUT /services/{id}/firewall/{ip}/rdns`                |
| `delete_rule(id, ip, rule_id)`                      | Kural sil                    | `DELETE /services/{id}/firewall/{ip}/rules/{rule_id}`  |
| `get_rules(id, ip)`                                 | Kuralları getir              | `GET /services/{id}/firewall/{ip}/rules`               |
| `create_rule(id, ip, data)`                         | Kural oluştur                | `POST /services/{id}/firewall/{ip}/rules`              |
| `get_statistics(id, ip)`                            | İstatistikleri getir         | `GET /services/{id}/firewall/{ip}/stats`               |

## 🔐 Güvenlik İpuçları

- Token'ı `.env` dosyasından veya ortam değişkeninden alın
- Her `ApiResponse` sonrası `.success` kontrolü yapın
- Hataları `match` veya `if let` ile detaylı yönetin

## 🚀 Performans Önerileri

- `tokio::join!` ile işlemleri paralel çalıştırın
- Gecikmeleri azaltmak için rate limit ayarlarını yapılandırın
- Uygulama başında istemciyi bir kere oluşturun ve tekrar kullanın

## 🧪 Test Örneği

```rust
#[tokio::test]
async fn test_user_service() {
    let token = std::env::var("NODESTY_TEST_TOKEN").unwrap();
    let client = NodestyApiClient::new(RestClientOptions::new(token));

    let response = client.user.get_current_user().await.unwrap();
    assert!(response.success);
    assert!(response.data.unwrap().email.contains("@"));
}
```

## 🐛 Sorun Giderme

**401 Unauthorized**

- Token geçersiz veya süresi dolmuş olabilir

**Timeout**

- Ağ bağlantısını ve timeout süresini kontrol edin

**Rate Limiting**

- `rate_limit_offset` değerini artırın veya daha az sıklıkla istek atın

## 📝 Changelog

### v1.0.0

- İlk stabil sürüm
- Tüm Nodesty API endpoint'leri destekleniyor
- Asenkron operasyonlar ve tip güvenliği

## 🤝 Katkıda Bulunma

1. Repo'yu forklayın
2. Branch oluşturun (`git checkout -b feature/harika-ozellik`)
3. Commit ve push yapın
4. PR gönderin

## 🔗 Bağlantılar

- [Nodesty Website](https://nodesty.com)
- [API Dökümantasyonu](https://nodesty.com/docs)

## ⭐ Destek

Bu proje faydalı olduysa ⭐ vererek destekleyebilirsiniz!

---

**Made with ❤️ for Nodesty Community.**

