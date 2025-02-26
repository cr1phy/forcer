use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    blazer::{
        AuthResponse, DeleteAccountRequest, DeleteSessionRequest, LoginRequest, LogoutRequest,
        SignupRequest, StatusResponse,
        account_service_server::AccountService as AccountServiceTrait,
    },
    entity::{
        prelude::User,
        user::{self, Column as UserColumn},
    },
    errors::ServerError,
    state::AppState,
    utils::jwt::{generate_token, validate_token},
};

pub struct AccountService {
    state: AppState,
}

impl AccountService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AccountServiceTrait for AccountService {
    async fn signup(
        &self,
        request: Request<SignupRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let db = &self.state.db;

        // Проверяем, существует ли уже пользователь
        let is_existed = User::find()
            .filter(
                UserColumn::Username
                    .eq(&req.username)
                    .or(UserColumn::Email.eq(&req.email)),
            )
            .one(db)
            .await
            .map_err(|_| ServerError::InternalError)?
            .is_some();

        if is_existed {
            return Err(ServerError::UserFound.into());
        }

        // Хэшируем пароль
        let hashed_password =
            hash(&req.password, DEFAULT_COST).map_err(|_| ServerError::InternalError)?;

        // Создаем нового пользователя
        let user_model = user::ActiveModel {
            email: Set(req.email),
            username: Set(req.username),
            password: Set(hashed_password.into_bytes()),
            created_at: Set(Utc::now().naive_utc()),
            last_online: Set(Utc::now().naive_utc()),
            is_active: Set(true),
            is_verified: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|_| ServerError::InternalError)?;

        // Генерируем токен
        let device_id = Uuid::now_v7().to_string();
        let token = generate_token(
            self.state.secret_key().to_string(),
            user_model.id,
            device_id.clone(),
        );

        // Формируем ответ
        let response = AuthResponse {
            token,
            session_id: device_id,
            success: true,
            error_message: String::new(),
        };

        Ok(Response::new(response))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let db = &self.state.db;

        // Ищем пользователя по email
        let user = User::find()
            .filter(user::Column::Email.eq(&req.email))
            .one(db)
            .await
            .map_err(|_| ServerError::InternalError)?;

        if user.is_none() {
            return Err(ServerError::UserNotFound.into());
        }

        let user = user.unwrap();

        // Проверяем пароль
        let password_valid = verify(
            &req.password,
            std::str::from_utf8(&user.password).map_err(|_| ServerError::InternalError)?,
        )
        .map_err(|_| ServerError::InternalError)?;

        if !password_valid {
            return Err(ServerError::InvalidCredentials.into());
        }

        // Генерируем токен и идентификатор сессии
        let device_id = Uuid::now_v7().to_string();
        let token = generate_token(
            self.state.secret_key().to_string(),
            user.id,
            device_id.clone(),
        );

        // Обновляем время последнего входа
        let mut user_model: user::ActiveModel = user.into();
        user_model.last_online = Set(Utc::now().naive_utc());
        user_model
            .update(db)
            .await
            .map_err(|_| ServerError::InternalError)?;

        // Формируем ответ
        let response = AuthResponse {
            token,
            session_id: device_id,
            success: true,
            error_message: String::new(),
        };

        Ok(Response::new(response))
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let req = request.into_inner();

        // Валидируем токен
        let token_data = validate_token(self.state.secret_key().to_string(), &req.token)
            .map_err(|_| ServerError::Unauthorized)?;

        // В реальном приложении здесь можно добавить логику для отзыва токена
        // Например, добавить его в черный список

        // Формируем ответ
        let response = StatusResponse {
            success: true,
            message: "Successfully logged out".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn delete_account(
        &self,
        request: Request<DeleteAccountRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let req = request.into_inner();
        let db = &self.state.db;

        // Валидируем токен
        let token_data = validate_token(self.state.secret_key().to_string(), &req.token)
            .map_err(|_| ServerError::Unauthorized)?;

        let user_id = token_data
            .sub
            .parse::<i32>()
            .map_err(|_| ServerError::Unauthorized)?;

        // Находим пользователя
        let user = User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|_| ServerError::InternalError)?;

        if user.is_none() {
            return Err(ServerError::UserNotFound.into());
        }

        // "Мягкое" удаление - помечаем как неактивного
        let mut user_model: user::ActiveModel = user.unwrap().into();
        user_model.is_active = Set(false);
        user_model
            .update(db)
            .await
            .map_err(|_| ServerError::InternalError)?;

        // Формируем ответ
        let response = StatusResponse {
            success: true,
            message: "Account successfully deleted".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn delete_session(
        &self,
        request: Request<DeleteSessionRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let req = request.into_inner();

        // Валидируем токен
        let token_data = validate_token(self.state.secret_key().to_string(), &req.token)
            .map_err(|_| ServerError::Unauthorized)?;

        // В реальном приложении здесь можно добавить логику для удаления сессии
        // Например, удалить запись о сессии из базы данных

        // Формируем ответ
        let response = StatusResponse {
            success: true,
            message: "Session successfully deleted".to_string(),
        };

        Ok(Response::new(response))
    }
}
