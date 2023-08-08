## Install

```sh
cargo install --git https://github.com/Setre14/cmt-rust.git
```


## Architecture

```mermaid
    class AppConfig {
        + u8 debug_level
        + String git_auto_sync
        + String system_config
    }

    class BaseConfig {
        + get_config() BaseConfig
        + save_config()
    }

    class SystemConfig {
        + Vec~String~ package_config
        + Vec~String~ env_config
        + String template_values
    }    
    
    class PackageConfig {
        + String pkgm
        + Vec~String~ packages

        + merge(config) PackageConfig
    }

    class EnvConfig {
        + Vec~String~ paths

        + merge(config) EnvConfig
    }

    class TemplateValues {
        + Dict::~String~ templates
    }

    AppConfig --* SystemConfig

    SystemConfig --|> BaseConfig : implements
    SystemConfig --* PackageConfig : 0..*
    SystemConfig --* EnvConfig : 0..*
    SystemConfig --* TemplateValues

    PackageConfig --|> BaseConfig : implements

    EnvConfig --|> BaseConfig : implements
```

