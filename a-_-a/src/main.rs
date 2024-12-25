use chrono::{Duration, NaiveTime, Weekday};
use num_traits::FromPrimitive;
use rand::Rng;

// Функция для расчета времени в пути
fn travel_time(start_time: NaiveTime, duration: Duration) -> NaiveTime {
    start_time + duration
}

// Функция для создания расписания занятий
fn class_schedule(
    start_time: NaiveTime,
    class_duration: Duration,
    break_duration: Duration,
    total_classes: usize,
) -> Vec<(NaiveTime, NaiveTime)> {
    let mut schedule = Vec::new();
    let mut current_time = start_time;

    for _ in 0..total_classes {
        let end_time = current_time + class_duration;
        schedule.push((current_time, end_time));
        current_time = end_time + break_duration;
    }

    schedule
}

// Функция для печати времени
fn print_time(label: &str, time: NaiveTime) {
    println!("{}: {}", label, time);
}

// Структура для представления ежедневного расписания
struct DailySchedule {
    wake_up_time: NaiveTime,
    travel_to_college: Option<NaiveTime>,
    classes: Vec<(NaiveTime, NaiveTime)>,
    travel_back_home: Option<NaiveTime>,
    long_travel_to_college: bool,
    long_travel_back_home: bool,
    mode: String,
    day: Weekday,
}

impl DailySchedule {
    // Конструктор для создания нового ежедневного расписания
    fn new(
        wake_up_time: NaiveTime,
        travel_to_college: Option<NaiveTime>,
        classes: Vec<(NaiveTime, NaiveTime)>,
        travel_back_home: Option<NaiveTime>,
        long_travel_to_college: bool,
        long_travel_back_home: bool,
        mode: String,
        day: Weekday,
    ) -> Self {
        DailySchedule {
            wake_up_time,
            travel_to_college,
            classes,
            travel_back_home,
            long_travel_to_college,
            long_travel_back_home,
            mode,
            day,
        }
    }

    // Метод для печати ежедневного расписания
    fn print(&self) {
        println!("\nDaily Schedule for {}: {}", self.day, self.mode);
        println!("====================");
        print_time("Wake up time", self.wake_up_time);
        if let Some(travel_to_college) = self.travel_to_college {
            println!(
                "Travel to college: {} (Long trip: {})",
                travel_to_college, self.long_travel_to_college
            );
        } else {
            println!("No need to travel to college today.");
        }
        println!("\nClasses:");
        for (index, (start, end)) in self.classes.iter().enumerate() {
            println!("Class {}: {} to {}", index + 1, start, end);
        }
        if let Some(travel_back_home) = self.travel_back_home {
            println!(
                "\nTravel back home: {} (Long trip: {})",
                travel_back_home, self.long_travel_back_home
            );
        } else {
            println!("\nNo need to travel back home today.");
        }
    }
}

fn main() {
    // Генерация случайного дня недели
    let mut rng = rand::thread_rng();
    let day_index = rng.gen_range(0..7);
    let day = Weekday::from_u32(day_index as u32 + 1).unwrap_or(Weekday::Mon);

    // Время в пути до колледжа (1.5 часа)
    let travel_duration = Duration::minutes(90);
    // Дополнительные 15 минут на дорогу до колледжа
    let additional_travel_time = Duration::minutes(15);

    // Параметры расписания занятий
    let class_duration = Duration::minutes(90);
    let break_duration = Duration::minutes(10);

    // Расписание занятий для каждого дня недели
    let schedules = [
        class_schedule(
            NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
        class_schedule(
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
        class_schedule(
            NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
        class_schedule(
            NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
        class_schedule(
            NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
        class_schedule(
            NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
        class_schedule(
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            class_duration,
            break_duration,
            6,
        ),
    ];

    // Выбор расписания для текущего дня
    let mut daily_schedule = schedules[day_index].clone();

    // Генерация случайного количества занятий (2-3)
    let num_classes = rng.gen_range(2..=3);
    let start_index = rng.gen_range(0..=daily_schedule.len() - num_classes);
    daily_schedule = daily_schedule
        .drain(start_index..start_index + num_classes)
        .collect::<Vec<_>>();

    // Определение режима обучения
    let mode = if day == Weekday::Tue {
        "Online"
    } else {
        "Offline"
    };

    // Время пробуждения, чтобы успеть на первое занятие
    let first_class_start = daily_schedule.first().unwrap().0;
    let wake_up_time = if mode == "Offline" {
        travel_time(
            first_class_start,
            -(travel_duration + additional_travel_time),
        )
    } else {
        first_class_start - Duration::minutes(30) // Проснуться за 30 минут до начала онлайн-занятия
    };

    let (travel_to_college, travel_back_home) = if mode == "Offline" {
        let travel_to_college = travel_time(wake_up_time, travel_duration + additional_travel_time);
        let travel_back_home = travel_time(daily_schedule.last().unwrap().1, travel_duration);
        (Some(travel_to_college), Some(travel_back_home))
    } else {
        (None, None)
    };

    // Указание, что поход в колледж и домой является долгим
    let long_travel_to_college = true;
    let long_travel_back_home = true;

    let daily_schedule = DailySchedule::new(
        wake_up_time,
        travel_to_college,
        daily_schedule,
        travel_back_home,
        long_travel_to_college,
        long_travel_back_home,
        mode.to_string(),
        day,
    );
    daily_schedule.print();
}

