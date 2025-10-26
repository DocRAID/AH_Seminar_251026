use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

// ==================== 시나리오 1: 큰 데이터 구조체 ====================
#[derive(Clone)]
struct LargeData {
    buffer: Vec<u8>,
    metadata: String,
    numbers: [i64; 1000],
}

impl LargeData {
    fn new(size: usize) -> Self {
        LargeData {
            buffer: vec![0u8; size],
            metadata: "Large metadata string".repeat(100),
            numbers: [42; 1000],
        }
    }
}

fn test_large_data_copy(iterations: usize, data_size: usize) -> u128 {
    let data = LargeData::new(data_size);
    let start = Instant::now();

    for _ in 0..iterations {
        let _copy1 = data.clone();
        let _copy2 = data.clone();
        let _copy3 = data.clone();
        let _copy4 = data.clone();
        let _copy5 = data.clone();
    }

    start.elapsed().as_millis()
}

fn test_large_data_rc(iterations: usize, data_size: usize) -> u128 {
    let data = Rc::new(LargeData::new(data_size));
    let start = Instant::now();

    for _ in 0..iterations {
        let _ref1 = Rc::clone(&data);
        let _ref2 = Rc::clone(&data);
        let _ref3 = Rc::clone(&data);
        let _ref4 = Rc::clone(&data);
        let _ref5 = Rc::clone(&data);
    }

    start.elapsed().as_millis()
}

// ==================== 시나리오 2: 읽기 전용 공유 데이터 ====================
struct Config {
    settings: Vec<String>,
    values: Vec<i32>,
}

impl Config {
    fn new() -> Self {
        Config {
            settings: (0..1000).map(|i| format!("setting_{}", i)).collect(),
            values: (0..1000).collect(),
        }
    }
}

fn test_config_copy(readers: usize) -> u128 {
    let config = Config::new();
    let start = Instant::now();

    let mut handles = vec![];

    for _ in 0..readers {
        let config_copy = Config {
            settings: config.settings.clone(),
            values: config.values.clone(),
        };

        handles.push(std::thread::spawn(move || {
            // 설정 읽기 시뮬레이션
            let _sum: i32 = config_copy.values.iter().sum();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    start.elapsed().as_millis()
}

fn test_config_arc(readers: usize) -> u128 {
    let config = Arc::new(Config::new());
    let start = Instant::now();

    let mut handles = vec![];

    for _ in 0..readers {
        let config_ref = Arc::clone(&config);

        handles.push(std::thread::spawn(move || {
            // 설정 읽기 시뮬레이션
            let _sum: i32 = config_ref.values.iter().sum();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    start.elapsed().as_millis()
}

// ==================== 시나리오 3: 캐시 시스템 ====================
use std::collections::HashMap;

struct CopyCacheEntry {
    data: Vec<u8>,
    timestamp: u64,
}

impl Clone for CopyCacheEntry {
    fn clone(&self) -> Self {
        CopyCacheEntry {
            data: self.data.clone(),
            timestamp: self.timestamp,
        }
    }
}

fn test_cache_copy(lookups: usize) -> u128 {
    let mut cache: HashMap<String, CopyCacheEntry> = HashMap::new();

    // 캐시 채우기
    for i in 0..100 {
        cache.insert(
            format!("key_{}", i),
            CopyCacheEntry {
                data: vec![0u8; 10_000],
                timestamp: i,
            },
        );
    }

    let start = Instant::now();

    for i in 0..lookups {
        let key = format!("key_{}", i % 100);
        if let Some(entry) = cache.get(&key) {
            // 데이터를 복사해서 사용
            let _copied = entry.clone();
        }
    }

    start.elapsed().as_millis()
}

fn test_cache_rc(lookups: usize) -> u128 {
    let mut cache: HashMap<String, Rc<CopyCacheEntry>> = HashMap::new();

    // 캐시 채우기
    for i in 0..100 {
        cache.insert(
            format!("key_{}", i),
            Rc::new(CopyCacheEntry {
                data: vec![0u8; 10_000],
                timestamp: i,
            }),
        );
    }

    let start = Instant::now();

    for i in 0..lookups {
        let key = format!("key_{}", i % 100);
        if let Some(entry) = cache.get(&key) {
            // Rc만 복사 (포인터 + 참조 카운트)
            let _reference = Rc::clone(entry);
        }
    }

    start.elapsed().as_millis()
}

// ==================== 메인 벤치마크 실행 ====================
fn main() {
    println!("{}", "=".repeat(70));

    // 시나리오 1: 큰 데이터
    println!("\n시나리오 1: 큰 데이터 구조체 (1MB) - 1000회 복사");
    let copy_time = test_large_data_copy(1000, 1_000_000);
    let rc_time = test_large_data_rc(1000, 1_000_000);
    println!("  Copy:  {}ms", copy_time);
    println!("  Rc:    {}ms", rc_time);
    println!(
        "  성능 향상: {:.2}x",
        copy_time as f64 / rc_time.max(1) as f64
    );

    // 시나리오 2: 멀티스레드 읽기
    println!("\n시나리오 2: 멀티스레드 설정 읽기 (50개 스레드)");
    let copy_time = test_config_copy(50);
    let arc_time = test_config_arc(50);
    println!("  Copy:  {}ms", copy_time);
    println!("  Arc:   {}ms", arc_time);
    println!(
        "  성능 향상: {:.2}x",
        copy_time as f64 / arc_time.max(1) as f64
    );

    // 시나리오 3: 캐시
    println!("\n시나리오 3: 캐시 조회 (10KB 엔트리, 10000회)");
    let copy_time = test_cache_copy(10000);
    let rc_time = test_cache_rc(10000);
    println!("  Copy:  {}ms", copy_time);
    println!("  Rc:    {}ms", rc_time);
    println!(
        "  성능 향상: {:.2}x",
        copy_time as f64 / rc_time.max(1) as f64
    );

    println!("{}", "\n".repeat(1) + &"=".repeat(70));
}
