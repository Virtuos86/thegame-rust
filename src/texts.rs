//! This module describes all text information showing in the game.

use hero;

pub const STR_HERO_EXP: &str = "Experience: ";
pub const STR_HERO_HP: &str = "Health/Max Health: ";
pub const STR_HERO_XY: &str = "Coordinates: ";

pub const STR_MONSTER1: &str = "Rat";
pub const STR_MONSTER2: &str = "Wolf";
pub const STR_MONSTER3: &str = "Giant Spider";
pub const STR_MONSTER4: &str = "Skeleton";
pub const STR_MONSTER5: &str = "Orc";
pub const STR_MONSTER6: &str = "Troll";
pub const STR_MONSTER7: &str = "Giant Snake";

pub const STR_TRAP: &str = "A trap works and damages your hero ";
pub const STR_LIVE: &str = "You regained your hero's health in the Source!";

pub const STR_HERO_DIED: &str = "Hero dies!";

pub const INIT_DIALOG: &str = "Select `Start` or press <Space> key to start playing.\
                               \nSelect `Quit` or press <Esc> key to exit.";
pub const HELP_EXIT_DIALOG: &str = "Select `Help` to get help.\n\
                              Select `Quit` to exit.";

pub fn help() -> String {
    format!("`@`: Hero\n\
             \n\
             Tiles:\n\
             `.`: Grass\n\
             `:`: Ground\n\
             `+`: Stairs Up\n\
             `-`: Stairs Down\n\
             `^`: Tree\n\
             `X`: Stone\n\
             \n\
             Monsters:\n\
             `p`: {}\n\
             `%`: {}\n\
             `!`: {}\n\
             `#`: {}\n\
             `&`: {}\n\
             `j`: {}\n\
             `A`: {}\n\
             \n\
             Shortcuts:\n\
             `w`, `a`, `s`, `d`: keys for moving of Hero\n\
             `e`: Hero's slots\n\
             `i`: Hero's items",
             STR_MONSTER1,
             STR_MONSTER2,
             STR_MONSTER3,
             STR_MONSTER4,
             STR_MONSTER5,
             STR_MONSTER6,
             STR_MONSTER7
    )
}

pub const STR_AXE: &str = "Axe";
pub const STR_SWORD: &str = "Sword";
pub const STR_BODYARMOR: &str = "Bodyarmor";
pub const STR_HELM: &str = "Helm";


pub const STR_TRAPOK: &str = "You have neutralized a trap!";
pub const STR_ADD_EXP: &str = "You increased your experience by ";
pub const STR_TRAPSKILL_OK: &str = "Your skill of neutralizing traps is increased.";

pub const STR_HERO_ITEMS: &str = "Hero's items";
pub const STR_EMPTY_ITEM: &str = "<empty>";
pub const STR_HERO_ITEMINFO: &str = "Press <Enter> to move the item to `Slots`";

pub const STR_HERO_SLOTITEMS: &str = "Used items";
pub const SlotName: [&str; hero::MaxSlots] = ["Body:    ", "In hand: "];
pub const STR_HERO_SLOTINFO: &str = "Press [<key>] to move the slot to `Items`";

pub const STR_GAME_OVER: &str = "









             #######          #####        ###     ###     ##########
           ###    ###        ### ###       ####   ####     ###    ###
         ###                ###   ###      ##### #####     ###
         ###     #####     ###     ###     ### ### ###     ######
         ###       ###     ###########     ###  #  ###     ###
           ###    ###      ###     ###     ###     ###     ###    ###
             #######       ###     ###     ###     ###     ##########
 
 
 
 
 
 
 
 
 
 
            #######        ###     ###     ##########     ########## 
         ###       ###     ###     ###     ###    ###     ###     ###
         ###       ###     ###     ###     ###            ###     ###
         ###       ###     ###     ###     ######         ##########
         ###       ###      ###   ###      ###            ###   ###
         ###       ###       ### ###       ###    ###     ###    ###
            #######           #####        ##########     ###     ###";