#![allow(unused)]

mod data
{
  pub struct UserId(pub String);

  pub struct PostId(pub String);

  pub struct GroupId(pub String);

  pub struct User
  {
    pub user_id: UserId,
    pub username: String,
    pub display_name: String,
  }

  pub struct Group
  {
    pub group_id: GroupId,
    pub group_name: String,
    pub description: String,
  }

  pub struct Post
  {
    pub post_id: PostId,
    pub author_id: UserId,
    pub group_id: Option<GroupId>,
    pub privacy: PostPrivacy,
    pub title: String,
    pub content: String,
  }

  pub enum PostPrivacy
  {
    Public,
    Private,
    GroupRead,
    GroupEdit,
  }
}

mod raw_query
{
  use super::data::*;

  pub struct DbError;

  pub fn get_user_info(user_id: &UserId) -> Result<User, DbError>
  {
    todo!()
  }

  pub fn get_user_groups(user_id: &UserId) -> Result<Vec<Group>, DbError>
  {
    todo!()
  }

  pub fn user_is_admin(user_id: &UserId) -> Result<bool, DbError>
  {
    todo!()
  }

  pub fn get_post_info(post_id: &PostId) -> Result<Post, DbError>
  {
    todo!()
  }
}

mod named_query
{
  use std::marker::PhantomData;

  use mononym::*;

  use super::{
    data::*,
    raw_query::{
      self,
      DbError,
    },
  };

  exists! {
      ExistUser(user: User) => UserHasId(user_id: UserId)
  }

  pub fn get_user_info<UserIdVal: HasType<UserId>>(
    seed: Seed<impl Name>,
    user_id: Named<UserIdVal, UserId>,
  ) -> Result<ExistUser<impl HasType<User>, UserIdVal>, DbError>
  {
    let user = raw_query::get_user_info(user_id.value())?;

    Ok(new_exist_user(seed, user))
  }

  exists! {
      ExistGroups(groups: Vec<Group>) => UserInGroups(user_id: UserId)
  }

  pub fn get_user_groups<UserIdVal: HasType<UserId>>(
    seed: Seed<impl Name>,
    user_id: &Named<UserIdVal, UserId>,
  ) -> Result<ExistGroups<impl HasType<Vec<Group>>, UserIdVal>, DbError>
  {
    let groups = raw_query::get_user_groups(user_id.value())?;

    Ok(new_exist_groups(seed, groups))
  }

  pub struct UserIsAdmin<UserIdVal>(PhantomData<UserIdVal>);

  pub fn user_is_admin<UserIdVal>(
    user_id: Named<UserIdVal, UserId>
  ) -> Result<Option<UserIsAdmin<UserIdVal>>, DbError>
  {
    let is_admin = raw_query::user_is_admin(user_id.value())?;

    if is_admin {
      Ok(Some(UserIsAdmin(PhantomData)))
    } else {
      Ok(None)
    }
  }

  exists! {
      ExistPost(post: Post) => PostHasId(post_id: PostId)
  }

  pub fn get_post_info<PostIdVal: HasType<PostId>>(
    seed: Seed<impl Name>,
    post_id: &Named<PostIdVal, PostId>,
  ) -> Result<ExistPost<impl HasType<Post>, PostIdVal>, DbError>
  {
    let post = raw_query::get_post_info(post_id.value())?;

    Ok(new_exist_post(seed, post))
  }
}

mod access_control
{}

fn main() {}
