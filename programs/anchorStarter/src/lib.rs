use anchor_lang::prelude::*;                                    // importing the anchor library

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");    //  the address or program id of your program

#[program]                                                      // program module and where the logic of the program lives
pub mod anchorStarter {
    use super::*;
    pub fn setup_platform(ctx: Context<TweetPlatform>) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;
        tweet.likes = 0;
        tweet.message = ("").to_string();
        Ok(())
    }

    pub fn write_tweet(
        ctx: Context<WriteTweet>,
        message: String,
        user_public_key: Pubkey
    ) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;

        if !tweet.message.trim().is_empty() {
            return Err(Errors::CannotUpdateTweet.into());
        }

        if message.trim().is_empty() {
            return Err(Errors::EmtpyMessage.into());
        }

        tweet.message = message;
        tweet.likes = 0;
        tweet.creator = user_public_key;

        Ok(())
    }

    pub fn like_tweet(ctx: Context<LikeTweet>, user_liking_tweet: Pubkey) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;

        if tweet.message.trim().is_empty() {
            return Err(Errors::NotValidTweet.into());
        }

        if tweet.likes == 5 {
            return Err(Errors::ReachedMaxLikes.into());
        }

        let mut iter = tweet.people_who_liked.iter();
        if iter.any(|&v| v == user_liking_tweet) {
            return Err(Errors::UserLikedTweet.into());
        }

        tweet.likes += 1;
        tweet.people_who_liked.push(user_liking_tweet);

        Ok(())
    }
}

#[derive(Accounts)]                                             // where the Accounts struct lives which is where accounts are validated
pub struct TweetPlatform<'info> {
    #[account(init, payer = user, space = 9000 )]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WriteTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}

#[derive(Accounts)]
pub struct LikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct Tweet {
    message: String,
    likes: u8,
    creator: Pubkey,
    people_who_liked: Vec<Pubkey>, // with  #[derive(Default)] we can assign default values
}


#[error]
pub enum Errors {
    #[msg("Tweet message cannot be updated")]
    CannotUpdateTweet,

    #[msg("Message cannot be empty")]
    EmtpyMessage,

    #[msg("Cannot receive more than 5 likes")]
    ReachedMaxLikes,

    #[msg("Cannot like a tweet without a valid message")]
    NotValidTweet,

    #[msg("User has already liked the tweet")]
    UserLikedTweet,
}
