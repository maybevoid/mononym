#![allow(unused)]

mod data
{
  #[derive(Debug, Clone, Eq, PartialEq)]
  pub struct UserId(pub String);

  #[derive(Debug, Clone, Eq, PartialEq)]
  pub struct PostId(pub String);

  #[derive(Debug, Clone, Eq, PartialEq)]
  pub struct GroupId(pub String);

  #[derive(Debug)]
  pub struct User
  {
    pub user_id: UserId,
    pub username: String,
    pub display_name: String,
  }

  #[derive(Debug)]
  pub struct Group
  {
    pub group_id: GroupId,
    pub group_name: String,
    pub description: String,
  }

  #[derive(Debug)]
  pub struct Post
  {
    pub post_id: PostId,
    pub author_id: UserId,
    pub group_id: Option<GroupId>,
    pub privacy: PostPrivacy,
    pub title: String,
    pub content: String,
  }

  #[derive(Debug)]
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
  use mononym::*;

  use super::{
    data::*,
    raw_query::{
      self,
      DbError,
    },
  };

  exists! {
    ExistUser(user: User) => UserHasId(user_id: UserId);

    ExistGroups(groups: Vec<Group>) => UserInGroups(user_id: UserId);

    ExistPost(post: Post) => PostHasId(post_id: PostId);
  }

  proof! {
    UserIsAdmin(user_id: UserId);
  }

  pub fn get_user_info<UserIdVal: HasType<UserId>>(
    seed: impl Seed,
    user_id: Named<UserIdVal, UserId>,
  ) -> Result<ExistUser<impl HasType<User>, UserIdVal>, DbError>
  {
    let user = raw_query::get_user_info(user_id.value())?;

    Ok(new_exist_user(seed, user))
  }

  pub fn get_user_groups<UserIdVal: HasType<UserId>>(
    seed: impl Seed,
    user_id: &Named<UserIdVal, UserId>,
  ) -> Result<ExistGroups<impl HasType<Vec<Group>>, UserIdVal>, DbError>
  {
    let groups = raw_query::get_user_groups(user_id.value())?;

    Ok(new_exist_groups(seed, groups))
  }

  pub fn user_is_admin<UserIdVal: HasType<UserId>>(
    user_id: Named<UserIdVal, UserId>
  ) -> Result<Option<UserIsAdmin<UserIdVal>>, DbError>
  {
    let is_admin = raw_query::user_is_admin(user_id.value())?;

    if is_admin {
      Ok(Some(UserIsAdmin::new()))
    } else {
      Ok(None)
    }
  }

  pub fn get_post_info<PostIdVal: HasType<PostId>>(
    seed: impl Seed,
    post_id: &Named<PostIdVal, PostId>,
  ) -> Result<ExistPost<impl HasType<Post>, PostIdVal>, DbError>
  {
    let post = raw_query::get_post_info(post_id.value())?;

    Ok(new_exist_post(seed, post))
  }
}

mod privacy
{

  use mononym::*;

  use super::{
    data::*,
    named_query::*,
  };

  pub struct Public;
  pub struct Private;
  pub struct GroupRead;
  pub struct GroupEdit;

  proof! {
    PostHasPrivacy<Privacy>(post_id: PostId);
  }

  pub enum SomePostPrivacy<PostIdVal: HasType<PostId>>
  {
    Public(PostHasPrivacy<Public, PostIdVal>),
    Private(PostHasPrivacy<Private, PostIdVal>),
    GroupRead(PostHasPrivacy<GroupRead, PostIdVal>),
    GroupEdit(PostHasPrivacy<GroupEdit, PostIdVal>),
  }

  pub fn check_post_privacy<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
    PostVal: HasType<Post>,
  >(
    post: &Named<PostVal, Post>,
    _post_has_id: &PostHasId<PostVal, PostIdVal>,
  ) -> SomePostPrivacy<PostIdVal>
  {
    match post.value().privacy {
      PostPrivacy::Public => SomePostPrivacy::Public(PostHasPrivacy::new()),
      PostPrivacy::Private => SomePostPrivacy::Private(PostHasPrivacy::new()),
      PostPrivacy::GroupRead => {
        SomePostPrivacy::GroupRead(PostHasPrivacy::new())
      }
      PostPrivacy::GroupEdit => {
        SomePostPrivacy::GroupEdit(PostHasPrivacy::new())
      }
    }
  }
}

mod access_control
{
  use mononym::*;

  use super::{
    data::*,
    named_query::*,
    privacy::*,
  };

  proof! {
    UserCanReadPost(post_id: PostId, user_id: UserId);

    UserCanEditPost(post_id: PostId, user_id: UserId);

    UserIsAuthor(post_id: PostId, user_id: UserId);

    UserInGroup(group_id: GroupId, user_id: UserId);
  }

  exists! {
    ExistPostGroup(group_id: GroupId) => PostInGroup(post_id: PostId);
  }

  pub fn check_user_is_author<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
    PostVal: HasType<Post>,
  >(
    user_id: &Named<UserIdVal, UserId>,
    post: &Named<PostVal, Post>,
    _post_has_id: &PostHasId<PostVal, PostIdVal>,
  ) -> Option<UserIsAuthor<PostIdVal, UserIdVal>>
  {
    if &post.value().author_id == user_id.value() {
      Some(UserIsAuthor::new())
    } else {
      None
    }
  }

  pub fn check_user_in_group<
    UserIdVal: HasType<UserId>,
    GroupIdVal: HasType<GroupId>,
    GroupsVal: HasType<Vec<Group>>,
  >(
    user_id: &Named<UserIdVal, UserId>,
    group_id: &Named<GroupIdVal, GroupId>,
    groups: &Named<GroupsVal, Vec<Group>>,
    _user_in_groups: &UserInGroups<GroupsVal, UserIdVal>,
  ) -> Option<UserInGroup<GroupIdVal, UserIdVal>>
  {
    for group in groups.value().iter() {
      if &group.group_id == group_id.value() {
        return Some(UserInGroup::new());
      }
    }

    None
  }

  pub fn get_post_group<PostIdVal: HasType<PostId>, PostVal: HasType<Post>>(
    seed: impl Seed,
    post: &Named<PostVal, Post>,
    _post_has_id: &PostHasId<PostVal, PostIdVal>,
  ) -> Option<ExistPostGroup<impl HasType<GroupId>, PostIdVal>>
  {
    post
      .value()
      .group_id
      .as_ref()
      .map(move |group_id| new_exist_post_group(seed, group_id.clone()))
  }

  pub fn check_post_in_group<
    GroupIdVal: HasType<GroupId>,
    PostIdVal: HasType<PostId>,
    PostVal: HasType<Post>,
  >(
    group_id: &Named<GroupIdVal, GroupId>,
    post: &Named<PostVal, Post>,
    _post_has_id: &PostHasId<PostVal, PostIdVal>,
  ) -> Option<PostInGroup<GroupIdVal, PostIdVal>>
  {
    if post.value().group_id.as_ref() == Some(group_id.value()) {
      Some(PostInGroup::new())
    } else {
      None
    }
  }

  pub fn author_can_edit_post<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
  >(
    _user_is_author: &UserIsAuthor<PostIdVal, UserIdVal>
  ) -> UserCanEditPost<PostIdVal, UserIdVal>
  {
    UserCanEditPost::new()
  }

  pub fn can_edit_also_can_read<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
  >(
    _can_edit: &UserCanEditPost<PostIdVal, UserIdVal>
  ) -> UserCanReadPost<PostIdVal, UserIdVal>
  {
    UserCanReadPost::new()
  }

  pub fn anyone_can_read_public_post<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
  >(
    _post_is_public: &PostHasPrivacy<Public, PostIdVal>
  ) -> UserCanReadPost<PostIdVal, UserIdVal>
  {
    UserCanReadPost::new()
  }

  pub fn admin_can_edit_any_post<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
  >(
    _user_is_admin: &UserIsAdmin<UserIdVal>
  ) -> UserCanEditPost<PostIdVal, UserIdVal>
  {
    UserCanEditPost::new()
  }
  pub fn group_member_can_read_post_with_group_read_privacy<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
    GroupIdVal: HasType<GroupId>,
  >(
    _user_in_group: &UserInGroup<GroupIdVal, UserIdVal>,
    _post_in_group: &PostInGroup<GroupIdVal, PostIdVal>,
    _post_has_group_read_privacy: &PostHasPrivacy<GroupRead, PostIdVal>,
  ) -> UserCanReadPost<PostIdVal, UserIdVal>
  {
    UserCanReadPost::new()
  }

  pub fn group_member_can_edit_post_with_group_edit_privacy<
    UserIdVal: HasType<UserId>,
    PostIdVal: HasType<PostId>,
    GroupIdVal: HasType<GroupId>,
  >(
    _user_in_group: &UserInGroup<GroupIdVal, UserIdVal>,
    _post_in_group: &PostInGroup<GroupIdVal, PostIdVal>,
    _post_has_group_read_privacy: &PostHasPrivacy<GroupEdit, PostIdVal>,
  ) -> UserCanEditPost<PostIdVal, UserIdVal>
  {
    UserCanEditPost::new()
  }
}

fn main() {}
