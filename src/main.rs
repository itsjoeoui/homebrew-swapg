use osascript::JavaScript;
use std::time::Duration;

use sysinfo::{RefreshKind, System, SystemExt};
use tokio::time::sleep;

async fn monitor() {
    loop {
        let system = System::new_with_specifics(RefreshKind::new().with_memory());
        let swap_used = system.used_swap();
        let mem_total = system.total_memory();

        if swap_used > mem_total {
            let script = format!(
                r#"
                var app = Application.currentApplication();
                app.includeStandardAdditions = true;
                app.displayNotification('Swap usage: {} GB', {{
                withTitle: 'High Swap Memory Usage',
                subtitle: 'Swap size has exceeded the physical memory size'
            }});"#,
                swap_used / 1024 / 1024 / 1024
            );
            let js = JavaScript::new(&script);
            let _: Result<(), _> = js.execute();
        } else {
            let script = format!(
                r#"
                var app = Application.currentApplication();
                app.includeStandardAdditions = true;
                app.displayNotification('Swap usage: {} GB', {{
                withTitle: 'Normal Swap Memory Usage',
                subtitle: 'Swap size is normal'
            }});"#,
                swap_used / 1024 / 1024 / 1024
            );
            let js = JavaScript::new(&script);
            let _: Result<(), _> = js.execute();
        }

        sleep(Duration::from_secs(10)).await;
    }
}

#[tokio::main]
async fn main() {
    monitor().await
}
