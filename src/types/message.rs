use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Базовый класс сообщения
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,                       // Идентификатор сообщения
    pub from: Uuid,                     // Идентификатор отправителя
    pub to: Uuid,                       // Идентификатор получателя
    pub timestamp: DateTime<Utc>,       // Время отправки сообщения
    pub text: Option<String>,           // Текст сообщения (не всегда присутствует)
    pub media: Option<MediaType>,       // Медиа тип сообщения (если есть)
    pub is_encrypted: bool,             // Флаг шифрования
    pub is_secret: bool,                // Флаг секретного чата
    pub delete_after: Option<u64>,      // Время самоуничтожения (в секундах)
    pub styles: Option<Vec<TextStyle>>, // Стили текста (если есть)
}

/// Разные типы медиа
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    Image(Vec<u8>), // Изображение (сжатое)
    Video(Vec<u8>), // Видео
    Voice(Vec<u8>), // Голосовое сообщение
    File(Vec<u8>),  // Файл
    Circle,         // Картинка или эмодзи в виде кружка
}

/// Стили текста для форматирования
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextStyle {
    Bold,   // Жирный текст
    Italic, // Курсив
    Code,   // Моноширенный текст
    Quote,  // Цитата
    Link,   // Ссылка
}

/// Класс для изображения
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageMessage {
    #[serde(flatten)]
    pub base: Message, // Наследуем от базового класса
    pub caption: Option<String>, // Подпись к изображению
}

impl ImageMessage {
    pub fn new(
        sender_id: Uuid,
        receiver_id: Uuid,
        image_data: Vec<u8>,
        caption: Option<String>,
    ) -> Self {
        Self {
            base: Message::new(
                sender_id,
                receiver_id,
                None,
                Some(MediaType::Image(image_data)),
            ),
            caption,
        }
    }
}

/// Класс для видео
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoMessage {
    #[serde(flatten)]
    pub base: Message, // Наследуем от базового класса
    pub caption: Option<String>, // Подпись к видео
}

impl VideoMessage {
    pub fn new(
        sender_id: Uuid,
        receiver_id: Uuid,
        video_data: Vec<u8>,
        caption: Option<String>,
    ) -> Self {
        Self {
            base: Message::new(
                sender_id,
                receiver_id,
                None,
                Some(MediaType::Video(video_data)),
            ),
            caption,
        }
    }
}

/// Класс для голосового сообщения
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceMessage {
    #[serde(flatten)]
    pub base: Message, // Наследуем от базового класса
    pub duration: u64, // Длительность голосового сообщения в секундах
}

impl VoiceMessage {
    pub fn new(sender_id: Uuid, receiver_id: Uuid, voice_data: Vec<u8>, duration: u64) -> Self {
        Self {
            base: Message::new(
                sender_id,
                receiver_id,
                None,
                Some(MediaType::Voice(voice_data)),
            ),
            duration,
        }
    }
}

/// Класс для простых файлов
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMessage {
    #[serde(flatten)]
    pub base: Message, // Наследуем от базового класса
    pub filename: String, // Имя файла
}

impl FileMessage {
    pub fn new(sender_id: Uuid, receiver_id: Uuid, file_data: Vec<u8>, filename: String) -> Self {
        Self {
            base: Message::new(
                sender_id,
                receiver_id,
                None,
                Some(MediaType::File(file_data)),
            ),
            filename,
        }
    }
}

/// Класс для "кружков" (эмодзи)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircleMessage {
    #[serde(flatten)]
    pub base: Message, // Наследуем от базового класса
    pub emoji: String, // Эмодзи
}

impl CircleMessage {
    pub fn new(sender_id: Uuid, receiver_id: Uuid, emoji: String) -> Self {
        Self {
            base: Message::new(sender_id, receiver_id, None, Some(MediaType::Circle)),
            emoji,
        }
    }
}

impl Message {
    /// Создание нового базового сообщения
    pub fn new(from: Uuid, to: Uuid, text: Option<String>, media: Option<MediaType>) -> Self {
        Self {
            id: Uuid::now_v7(),
            from,
            to,
            timestamp: Utc::now(),
            text,
            media,
            is_encrypted: true,
            is_secret: false,
            delete_after: None,
            styles: None,
        }
    }

    /// Функция для добавления стилей текста
    pub fn add_styles(&mut self, styles: Vec<TextStyle>) {
        self.styles = Some(styles);
    }

    /// Функция для включения или выключения секретного чата
    pub fn set_secret(&mut self, is_secret: bool) {
        self.is_secret = is_secret;
    }

    /// Функция для настройки времени самоуничтожения
    pub fn set_delete_after(&mut self, time: u64) {
        self.delete_after = Some(time);
    }
}