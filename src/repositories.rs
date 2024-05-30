use crate::models::*;
use crate::schema::*;
use diesel::dsl::now;
use diesel::dsl::IntervalDsl;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).get_results(conn).await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(conn)
            .await
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id))
            .execute(conn)
            .await
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).load(conn).await
    }

    pub async fn find_since(
        conn: &mut AsyncPgConnection,
        hours_since: i32,
    ) -> QueryResult<Vec<Crate>> {
        crates::table.filter(
            crates::created_at.ge(now - hours_since.hours())
        ).load(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(conn)
            .await
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: i32,
        a_crate: Crate,
    ) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn).await
    }
}

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user: User = diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .await?;

        for role_code in role_codes {
            let new_user_role = {
                // Check if role_code exists, we set user_id = user.id (users table)
                // and role_id = role.id (roles table)
                if let Ok(role) = RoleRepositoty::find_by_code(conn, &role_code).await {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    // if role does not exist, we create new_role on roles table
                    // and repeat above step
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        code: role_code,
                        name
                    };
                    let role = RoleRepositoty::create(conn, new_role).await?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(conn)
                .await?;
        }
        Ok(user)
    }

    pub async fn find_with_roles(conn: &mut AsyncPgConnection) 
    -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(conn).await.unwrap();
        let join_result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole,Role)>(conn)
            .await?
            .grouped_by(&users);

        let result = users.into_iter().zip(join_result).collect();
        Ok(result)
    }
    pub async fn find_by_username(conn: &mut AsyncPgConnection, username: &String) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).get_result(conn).await
    }
    
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(conn).await
    }
    
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(
            users_roles::table.filter(users_roles::user_id.eq(id)))
            .execute(conn)
            .await?;
        diesel::delete(users::table.find(id)).execute(conn).await
    }
    
}

pub struct RoleRepositoty;

impl RoleRepositoty {
    pub async fn create(conn: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(conn)
            .await
    }

    pub async fn find_by_code(conn: &mut AsyncPgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(conn).await
    }

    pub async fn find_by_id(conn: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(conn).await
    }

    pub async fn find_by_user(conn: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        // An user finds all role_id in users_roles table
        // that returns Vec<role_id>, so we can find all roles in the roles table
        let user_roles: Vec<UserRole> = UserRole::belonging_to(&user).get_results(conn).await?;
        let user_role_ids: Vec<i32> = user_roles.iter()
            .map(|ur| ur.role_id)
            .collect();

        Self::find_by_id(conn, user_role_ids).await
    }
}