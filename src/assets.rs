use ggez::{graphics, Context, GameResult};

//Идея за struct Assets: https://github.com/AndrewRadev/rust-shooter/blob/main/src/assets.rs
pub struct Assets {
    pub aceHeart: graphics::Image,
    pub twoHeart: graphics::Image,
    pub threeHeart: graphics::Image,
    pub fourHeart: graphics::Image,
    pub fiveHeart: graphics::Image,
    pub sixHeart: graphics::Image,
    pub sevenHeart: graphics::Image,
    pub eightHeart: graphics::Image,
    pub nineHeart: graphics::Image,
    pub tenHeart: graphics::Image,
    pub jackHeart: graphics::Image,
    pub queenHeart: graphics::Image,
    pub kingHeart: graphics::Image,

    pub aceDiamond: graphics::Image,
    pub twoDiamond: graphics::Image,
    pub threeDiamond: graphics::Image,
    pub fourDiamond: graphics::Image,
    pub fiveDiamond: graphics::Image,
    pub sixDiamond: graphics::Image,
    pub sevenDiamond: graphics::Image,
    pub eightDiamond: graphics::Image,
    pub nineDiamond: graphics::Image,
    pub tenDiamond: graphics::Image,
    pub jackDiamond: graphics::Image,
    pub queenDiamond: graphics::Image,
    pub kingDiamond: graphics::Image,

    pub aceClub: graphics::Image,
    pub twoClub: graphics::Image,
    pub threeClub: graphics::Image,
    pub fourClub: graphics::Image,
    pub fiveClub: graphics::Image,
    pub sixClub: graphics::Image,
    pub sevenClub: graphics::Image,
    pub eightClub: graphics::Image,
    pub nineClub: graphics::Image,
    pub tenClub: graphics::Image,
    pub jackClub: graphics::Image,
    pub queenClub: graphics::Image,
    pub kingClub: graphics::Image,

    pub aceSpade: graphics::Image,
    pub twoSpade: graphics::Image,
    pub threeSpade: graphics::Image,
    pub fourSpade: graphics::Image,
    pub fiveSpade: graphics::Image,
    pub sixSpade: graphics::Image,
    pub sevenSpade: graphics::Image,
    pub eightSpade: graphics::Image,
    pub nineSpade: graphics::Image,
    pub tenSpade: graphics::Image,
    pub jackSpade: graphics::Image,
    pub queenSpade: graphics::Image,
    pub kingSpade: graphics::Image,
    pub back: graphics::Image,

    pub frame: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let aceHeart = graphics::Image::from_path(ctx, "/ace_of_hearts.png")?;
        let twoHeart = graphics::Image::from_path(ctx, "/2_of_hearts.png")?;
        let threeHeart = graphics::Image::from_path(ctx, "/3_of_hearts.png")?;
        let fourHeart = graphics::Image::from_path(ctx, "/4_of_hearts.png")?;
        let fiveHeart = graphics::Image::from_path(ctx, "/5_of_hearts.png")?;
        let sixHeart = graphics::Image::from_path(ctx, "/6_of_hearts.png")?;
        let sevenHeart = graphics::Image::from_path(ctx, "/7_of_hearts.png")?;
        let eightHeart = graphics::Image::from_path(ctx, "/8_of_hearts.png")?;
        let nineHeart = graphics::Image::from_path(ctx, "/9_of_hearts.png")?;
        let tenHeart = graphics::Image::from_path(ctx, "/10_of_hearts.png")?;
        let jackHeart = graphics::Image::from_path(ctx, "/jack_of_hearts.png")?;
        let queenHeart = graphics::Image::from_path(ctx, "/queen_of_hearts.png")?;
        let kingHeart = graphics::Image::from_path(ctx, "/king_of_hearts.png")?;

        let aceDiamond = graphics::Image::from_path(ctx, "/ace_of_diamonds.png")?;
        let twoDiamond = graphics::Image::from_path(ctx, "/2_of_diamonds.png")?;
        let threeDiamond = graphics::Image::from_path(ctx, "/3_of_diamonds.png")?;
        let fourDiamond = graphics::Image::from_path(ctx, "/4_of_diamonds.png")?;
        let fiveDiamond = graphics::Image::from_path(ctx, "/5_of_diamonds.png")?;
        let sixDiamond = graphics::Image::from_path(ctx, "/6_of_diamonds.png")?;
        let sevenDiamond = graphics::Image::from_path(ctx, "/7_of_diamonds.png")?;
        let eightDiamond = graphics::Image::from_path(ctx, "/8_of_diamonds.png")?;
        let nineDiamond = graphics::Image::from_path(ctx, "/9_of_diamonds.png")?;
        let tenDiamond = graphics::Image::from_path(ctx, "/10_of_diamonds.png")?;
        let jackDiamond = graphics::Image::from_path(ctx, "/jack_of_diamonds.png")?;
        let queenDiamond = graphics::Image::from_path(ctx, "/queen_of_diamonds.png")?;
        let kingDiamond = graphics::Image::from_path(ctx, "/king_of_diamonds.png")?;

        let aceClub = graphics::Image::from_path(ctx, "/ace_of_clubs.png")?;
        let twoClub = graphics::Image::from_path(ctx, "/2_of_clubs.png")?;
        let threeClub = graphics::Image::from_path(ctx, "/3_of_clubs.png")?;
        let fourClub = graphics::Image::from_path(ctx, "/4_of_clubs.png")?;
        let fiveClub = graphics::Image::from_path(ctx, "/5_of_clubs.png")?;
        let sixClub = graphics::Image::from_path(ctx, "/6_of_clubs.png")?;
        let sevenClub = graphics::Image::from_path(ctx, "/7_of_clubs.png")?;
        let eightClub = graphics::Image::from_path(ctx, "/8_of_clubs.png")?;
        let nineClub = graphics::Image::from_path(ctx, "/9_of_clubs.png")?;
        let tenClub = graphics::Image::from_path(ctx, "/10_of_clubs.png")?;
        let jackClub = graphics::Image::from_path(ctx, "/jack_of_clubs.png")?;
        let queenClub = graphics::Image::from_path(ctx, "/queen_of_clubs.png")?;
        let kingClub = graphics::Image::from_path(ctx, "/king_of_clubs.png")?;

        let aceSpade = graphics::Image::from_path(ctx, "/ace_of_spades.png")?;
        let twoSpade = graphics::Image::from_path(ctx, "/2_of_spades.png")?;
        let threeSpade = graphics::Image::from_path(ctx, "/3_of_spades.png")?;
        let fourSpade = graphics::Image::from_path(ctx, "/4_of_spades.png")?;
        let fiveSpade = graphics::Image::from_path(ctx, "/5_of_spades.png")?;
        let sixSpade = graphics::Image::from_path(ctx, "/6_of_spades.png")?;
        let sevenSpade = graphics::Image::from_path(ctx, "/7_of_spades.png")?;
        let eightSpade = graphics::Image::from_path(ctx, "/8_of_spades.png")?;
        let nineSpade = graphics::Image::from_path(ctx, "/9_of_spades.png")?;
        let tenSpade = graphics::Image::from_path(ctx, "/10_of_spades.png")?;
        let jackSpade = graphics::Image::from_path(ctx, "/jack_of_spades.png")?;
        let queenSpade = graphics::Image::from_path(ctx, "/queen_of_spades.png")?;
        let kingSpade = graphics::Image::from_path(ctx, "/king_of_spades.png")?;
        let back = graphics::Image::from_path(ctx, "/back.png")?;
        let frame = graphics::Image::from_path(ctx, "/frame.png")?;
        Ok(Assets {
            aceHeart,
            twoHeart,
            threeHeart,
            fourHeart,
            fiveHeart,
            sixHeart,
            sevenHeart,
            eightHeart,
            nineHeart,
            tenHeart,
            jackHeart,
            queenHeart,
            kingHeart,
            aceDiamond,
            twoDiamond,
            threeDiamond,
            fourDiamond,
            fiveDiamond,
            sixDiamond,
            sevenDiamond,
            eightDiamond,
            nineDiamond,
            tenDiamond,
            jackDiamond,
            queenDiamond,
            kingDiamond,
            aceClub,
            twoClub,
            threeClub,
            fourClub,
            fiveClub,
            sixClub,
            sevenClub,
            eightClub,
            nineClub,
            tenClub,
            jackClub,
            queenClub,
            kingClub,
            aceSpade,
            twoSpade,
            threeSpade,
            fourSpade,
            fiveSpade,
            sixSpade,
            sevenSpade,
            eightSpade,
            nineSpade,
            tenSpade,
            jackSpade,
            queenSpade,
            kingSpade,
            back,
            frame,
        })
    }
}
