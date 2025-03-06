use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::{thread, time::Duration};
use local_ip_address::local_ip;

pub fn start_mdns_server() {
    println!("skac");
    // Создаем экземпляр mDNS демона
    let mdns = ServiceDaemon::new().expect("Не удалось создать mDNS демон");

    // Регистрируем службу
    let service_type = "_http._tcp.local.";
    let instance_name = "my_service";
    let port = 8080;
    let properties = [("path", "/")];
    let ip = local_ip().expect("Cant detect ip address");

    println!("Id address of host is: {}" ,ip);
    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
        "my_host.local.",
        ip,
        port,
        &properties[..],
    )
    .expect("Не удалось создать информацию о службе");

    mdns.register(my_service)
        .expect("Не удалось зарегистрировать службу");

    // Слушаем события mDNS
    let receiver = mdns
        .browse(service_type)
        .expect("Не удалось начать просмотр");

    while let Ok(event) = receiver.recv_timeout(Duration::from_secs(1001)) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                println!("Обнаружена служба: {:?}", info);
            }
            _ => {
                // println!("Другое событие: {:?}", event);
            }
        }
    }

    thread::sleep(Duration::from_millis(500));
}
