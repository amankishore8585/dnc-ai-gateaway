pub struct Backend {
    pub addr: String,
    pub healthy: bool,
}

pub struct LoadBalancer {
    pub backends: Vec<Backend>,
    pub index: usize,
}

impl LoadBalancer {
    pub fn new(addrs: Vec<String>) -> Self {
        let backends = addrs
            .into_iter()
            .map(|addr| Backend { addr, healthy: true })
            .collect();

        Self { backends, index: 0 }
    }

    pub fn next(&mut self) -> Option<String> {
        let len = self.backends.len();

        for _ in 0..len {
            let backend = &mut self.backends[self.index];
            self.index = (self.index + 1) % len;

            if backend.healthy {
                return Some(backend.addr.clone());
            }
        }

        None
    }

    pub fn mark_unhealthy(&mut self, addr: &str) {
        for backend in &mut self.backends {
            if backend.addr == addr {
                backend.healthy = false;
            }
        }
    }
}